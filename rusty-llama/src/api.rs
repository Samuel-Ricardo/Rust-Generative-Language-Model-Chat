use crate::model::conversation::Conversation;
use cfg_if::cfg_if;
use leptos::*;

#[server(Converse, "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use actix_web::web::Query;
    use leptos_actix::extract;
    use llm::models::Llama;

    let model: Data<Llama> = extract().await.unwrap();

    use llm::KnownModel;
    let CHARACTER_NAME = "### Assistant";
    let USER_NAME = "### Human";
    let PERSONA = "A Chat between a human and an assistant";
    let mut history = format!(
        "{CHARACTER_NAME}:Hello - How may I help you today?\n\
        {USER_NAME}:What is the capital of France?\n\
        {CHARACTER_NAME}:Paris is the capital of France.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let current_line = if message.user {
            format!("{CHARACTER_NAME}:{msg}\n")
        } else {
            format!("{USER_NAME}:{msg}\n")
        };

        history.push_str(&current_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());

    session
        .infer(
            model.as_ref(),
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{PERSONA}\n{history}\n{CHARACTER_NAME}:")
                    .as_str()
                    .into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(USER_NAME), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));

    Ok(res)
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(&t);
                    } else {
                        out_str.push_str(&reverse_buf);
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
