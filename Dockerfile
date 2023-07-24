FROM rust:1.71

WORKDIR /app
COPY . .
WORKDIR /app/api
RUN cargo build --release

ENV ROCKET_PORT=443
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_CLI_COLORS=false
CMD ["cargo", "run", "--release"]
EXPOSE 443
