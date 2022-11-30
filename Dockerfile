FROM rust:1.63.0

WORKDIR /usr/src
COPY . .

RUN rustup component add rustfmt
RUN cargo build

EXPOSE 4000

CMD ["cargo", "run", "--", "--config", "router.yaml", "--supergraph", "supergraph-schema.graphql"]