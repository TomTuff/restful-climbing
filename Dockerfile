FROM rust:1.68-alpine3.16 as builder

# https://stackoverflow.com/questions/72461117/error-failed-to-run-custom-build-command-for-ring-v0-16-20
RUN apk update && \
    apk upgrade
RUN apk add --no-cache musl-dev

RUN USER=root cargo new --bin restful-climbing
WORKDIR /restful-climbing
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env
RUN cargo build --release 
RUN rm ./src/*.rs

ADD ./src ./src

RUN rm ./target/release/deps/restful_climbing*
RUN cargo build --release

FROM ubuntu:22.04
ARG APP=/usr/src/app
EXPOSE 8080
ENV APP_USER=appusr

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /restful-climbing/target/release/restful-climbing ${APP}/restful-climbing

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./restful-climbing"]