FROM rust:1.67.1-slim-bullseye

WORKDIR /app

RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
ENV TZ=Asia/Tokyo
RUN echo "Asia/Tokyo" > /etc/timezone

RUN apt update -y && apt upgrade -y
RUN apt install -y default-mysql-client default-libmysqlclient-dev
RUN apt install -y pkg-config

RUN cargo install sqlx-cli
RUN rustup component add rustfmt
RUN rustup component add clippy