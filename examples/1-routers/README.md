# Routers

Showcase of Zenoh routers.

## Structure

Basic two-binary pub/sub communication, but the catch is that they are on different LANs connected through a router (simulated with docker).

## Run

### Docker compose (Required)

```sh
# add --build to ensure rebuilding on change
# use --scale to select how many replicas of each service to start (default: 1)

docker compose up
```

See: [compose.yaml](./compose.yaml)

## Considerations

Routers are essential not only to expand the network of peers outside the LAN, but also to allow clients to connect and interact with the network (clients could connect directly to one peer, but it is typically easier to put a router as a middleware).

### Plugins

Routers also provide some useful plugins:

- [REST Plugin](https://zenoh.io/docs/manual/plugin-http/): allows external applications to interact with the network through HTTP.
- [Storage Plugin](https://zenoh.io/docs/manual/plugin-storage-manager/): allows routers to store data on different types of storages (e.g. filesystem, S3, etc.).

See: [zenoh plugins](https://zenoh.io/docs/manual/plugins/)
