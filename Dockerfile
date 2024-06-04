FROM golang:1.22.3-alpine as builder

WORKDIR /app
COPY . .

RUN cd golang && go build

FROM node:22.2.0-alpine as run

WORKDIR /app

COPY . .

RUN cd svelte && yarn install && yarn run build && cd .. \
    && yarn cache clean

COPY exec_all.sh exec_all.sh
RUN chmod +x exec_all.sh

COPY --from=builder /app/golang/brainfuck /app/brainfuck

CMD ["sh", "exec_all.sh"]
