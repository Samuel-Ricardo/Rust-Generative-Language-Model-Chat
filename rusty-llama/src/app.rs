use leptos::*;
use leptos_meta::*;

use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // React | useState()
    let (conversation, set_conversation) = create_signal(Conversation::new());

    // React | useCallBack
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };

        set_conversation.update(move |c| c.messages.push(user_message));

        // TODO - converse
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rusty-llama.css"/>

        // sets the document title
        <Title text="Rusty Llama"/>
        <ChatArea conversation />
        <TypeArea send />
    }
}
