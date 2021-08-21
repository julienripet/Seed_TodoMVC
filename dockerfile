FROM rust:1.54

WORKDIR /usr/src/counter

COPY . .

RUN cargo install cargo-make

EXPOSE 8000
