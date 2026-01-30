# Hello Zenoh

Basic implementation of the two zenoh communication models.

## Structure

### Pub/sub

The pub publishes a basic "hello" message to an example key a number of times, while the sub listens for new values on the same key.

There are two binaries involved:

- [`pub`](./src/bin/pub.rs): Publishes a message to the topic (key).
- [`sub`](./src/bin/sub.rs): Subscribes to the topic and prints the received data.

See: [zenoh pub/sub rust api example](https://docs.rs/zenoh/latest/zenoh/index.html#publishingsubscribing)

### Query/reply

The queryable serves a basic "hello" message to an example key a number of times, while the querier requests the data until the queryable is disconnected.

There are two binaries involved:

- [`queryable`](./src/bin/queryable.rs): Provides data at the topic (key).
- [`querier`](./src/bin/querier.rs): Queries data at the topic and prints the received reply.

See: [zenoh query/reply rust api example](https://docs.rs/zenoh/latest/zenoh/index.html#queryreply-1)

## Run

### Docker compose (Suggested)

Select manually how many replicas of each service to start with the `--scale` flag or its corresponding env variable:

- pub: `HELLO_ZENOH_PUB_REPLICAS`
- sub: `HELLO_ZENOH_SUB_REPLICAS`
- queryable: `HELLO_ZENOH_QUERYABLE_REPLICAS`
- querier: `HELLO_ZENOH_QUERIER_REPLICAS`

```sh
# add --build to ensure rebuilding on change

docker compose up --scale pub=1 --scale sub=1
# or
# docker compose up --scale queryable=1 --scale querier=1
```

See: [compose.yaml](./compose.yaml)

### Manual

```sh
cargo build --bins # build everything first

cargo run --bin pub &
cargo run --bin sub &
# or
# cargo run --bin queryable &
# cargo run --bin querier &

(sleep 30 && jobs -p | xargs -r kill) & # kill after 30 seconds to prevent remain stuck
```

## Considerations

### Deployment

By default, zenoh uses peer to peer communication to make all application talk directly to each other, leveraging multicast and gossip scouting to discover new peers and routers in the network.
When increasing the number of peers, a peer mesh should be considered to avoid having too many connections open.

When applications run on the same host, zenoh optimized inter process communication using pipes instead of the network.

See: [zenoh deployment modes](https://zenoh.io/docs/getting-started/deployment/)
See: [zenoh ultra low-latency](https://zenoh.io/blog/2023-10-03-zenoh-dragonite/#support-for-ultra-low-latency)
