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
