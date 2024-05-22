FROM rust:1.78 as builder

WORKDIR /usr/src/brain_fuck_interpreter
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim as run

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/brain_fuck_interpreter /usr/local/bin/brain_fuck_interpreter

CMD ["brain_fuck_interpreter"]
