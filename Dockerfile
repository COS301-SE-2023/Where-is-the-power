FROM rust:1.71

RUN apt-get update
RUN apt-get install -y python3-pip
RUN pip install requests
WORKDIR /app
COPY . .
WORKDIR /app/api
RUN cargo build --release

ENV ROCKET_PORT=443
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_CLI_COLORS=false
CMD ["target/release/api"]
EXPOSE 443
