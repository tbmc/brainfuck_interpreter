FROM rust:1.78 as builder

WORKDIR /usr/src/brain_fuck_interpreter
COPY . .

RUN cargo install --path .

FROM node:22.2.0-bookworm as run

WORKDIR /app

COPY . .

RUN cd svelte && yarn install && yarn run build && cd ..
RUN cd websocketServer && yarn install && cd ..

COPY exec_all.sh exec_all.sh
RUN chmod +x exec_all.sh

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/brain_fuck_interpreter /usr/local/bin/brain_fuck_interpreter

CMD ["bash", "exec_all.sh"]
