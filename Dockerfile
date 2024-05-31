FROM rust:1.78-slim-buster as builder

WORKDIR /usr/src/brainfuck_interpreter
COPY . .

RUN cd rust && rustup target add x86_64-unknown-linux-musl \
    && cargo install --target=x86_64-unknown-linux-musl --path .

FROM node:22.2.0-alpine as run

WORKDIR /app

COPY . .

RUN cd svelte && yarn install && yarn run build && cd .. \
    && cd websocketServer && yarn install && cd .. \
    && yarn cache clean

COPY exec_all.sh exec_all.sh
RUN chmod +x exec_all.sh

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/brainfuck_interpreter /usr/local/bin/brainfuck_interpreter

CMD ["sh", "exec_all.sh"]
