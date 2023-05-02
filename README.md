# confls

CMD utility to create, restore and swap configuration files easily.

## Development

1. Run `docker compose up -d` from this directory.
   - This will build the image and start the container as a daemon.
   - You can also run the [`start.fish`](./start.fish) script.
   - To easily remove the container and images, run the [`remove.fish`](./remove.fish) script.
2. Run `docker exec -it confls-arch-dev fish` to enter the container.
3. Code is mounted to `/code/confls` as a volume inside the container.
4. Build could be done to another directory by `cargo build --target-dir /home/test/builds` to
   avoid overwriting local builds from docker.

## Specification

- [Environment](./specs/environment.md)
