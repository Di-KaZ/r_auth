FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/auth_api

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

EXPOSE 8000 

RUN apt-get update -y && apt-get install postgresql -y

COPY --from=builder /usr/local/cargo/bin/auth_api /usr/local/bin/auth_api

CMD ["auth_api"]
