# Axum-Tailwind-Template
A simple template using Axum and Tailwind to speed up your Rust development. Deploy-ready with a Dockerfile
so that you can host this anywhere you can a container.

## Stack
- [Axum](https://docs.rs/axum/latest/axum/)
- [Twailwind CSS](https://tailwindcss.com/)

## Developing
- [Install and config](https://tailwindcss.com/blog/standalone-cli) the Tailwindcss cli.
- Install [cargo-watch](https://crates.io/crates/cargo-watch): 
`cargo install cargo-watch`
- Install [just](https://github.com/casey/just#packages)

Included is a bash script in a `justfile` that can be run with:
```shell
just dev
```

This will start the Axum server and Tailwind binary in watch modes so that saves
will trigger rebuilds while you're developing. On exiting this process, the Tailwind
binary will minify its outputted css.

### Building the Tailwind CSS separately 
- Run:
```shell
just build-tailwind
```

- Or as developing, in another tab run:
```shell
just run-tailwind
```
to automatically compile the tailwind as you're making changes.
