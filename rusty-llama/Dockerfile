FROM rust:1.77-slim as builder

RUN cargo install cargo-leptos &&\
  rustup target add wasm32-unknown-unknown &&\
  apt update -y && apt upgrade -y &&\
  apt install nodejs npm -y


WORKDIR /app

COPY . .
RUN npm ci && cargo leptos build -r -vv

FROM rust:1.77-slim 

WORKDIR /app

COPY --from=builder /app/target/release/rusty-llama ./rusty-llama
COPY --from=builder /app/target/site ./site

ARG LLAMA_MODEL_PATH
ENV LLAMA_MODEL_PATH=$LLAMA_MODEL_PATH

EXPOSE 3000

CMD ["./rusty-llama"]

