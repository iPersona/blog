# Docker

## Q&A

### How to access host machine from docker container

- macOS: `http://docker.for.mac.localhost:8888`
- linux: `http://172.17.0.1:8888` (Get this IP of interface `docker0` through `ifconfig` command)

#### Refs

- [Accessing host machine from within docker container Docker Desktop for Mac docker](https://forums.docker.com/t/accessing-host-machine-from-within-docker-container/14248/15)