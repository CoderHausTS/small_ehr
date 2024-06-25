FROM rust
WORKDIR /code
EXPOSE 3000
COPY . .

# we need diesel cli to run our migrations
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

RUN cargo build

# this might be better served with diesel embed_migrations macro
CMD bash -c "diesel setup && cargo run -- run"

