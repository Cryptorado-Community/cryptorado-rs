# Cryptorado Community Home

> HEAVY WIP

See the [github project](https://github.com/orgs/Cryptorado-Community/projects/1) for infomration on what we are up to.

## Development

Installation is made easy with [cargo make](https://sagiegurari.github.io/cargo-make/) - install that utility first, then see the commands via:

```sh
# Install
cargo install cargo-make

# Show all make tasks
makers --list-all-steps
```

Run locally with:

```sh
# With Make
makers s

# With shuttle installed
cargo shuttle run

# Without shuttle at all
cargo run --features local
```

## Deployment

TODO

## Credits

This service was developed based on the [static-file-server](https://github.com/shuttle-hq/shuttle-examples/tree/main/axum/static-files).
