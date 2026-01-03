# Example pub/sub with Zenoh

https://zenoh.io/docs/getting-started/first-app/

## Structure

There are two rust binaries in this example:

- [`pub`](./src/bin/pub.rs): Publishes a message to a topic.
- [`sub`](./src/bin/sub.rs): Subscribes to a topic and prints the received messages.
