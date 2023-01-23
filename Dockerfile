FROM rust:1.66.1

WORKDIR /usr/src
COPY . .

RUN apt-get update
RUN apt-get -y install \
    protobuf-compiler

RUN rustup component add rustfmt
RUN cargo build

EXPOSE 4000

CMD ["cargo", "run", "--", "--config", "router.yaml", "--supergraph", "supergraph-schema.graphql"]