FROM rust:1.70

RUN apt-get update
RUN apt-get install -y nodejs
RUN apt-get install -y npm
WORKDIR /app
COPY . .
WORKDIR /app/api
RUN cargo build --release

ENV ROCKET_PORT=443
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["cargo", "run", "--release"]
EXPOSE 443
