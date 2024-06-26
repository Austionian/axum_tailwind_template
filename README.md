# Axum-Tailwind-Template
A simple template using Axum and Tailwind to speed up your Rust development. Deploy-ready with a Dockerfile
so that you can host this anywhere you can a container.

No Node/npm needed!

_Based on my [blog](https://r00ks.io) ([code here](https://github.com/Austionian/bl0g))._

## Stack
- [Axum](https://docs.rs/axum/latest/axum/)
- [Twailwind CSS](https://tailwindcss.com/)
- [Tera](https://keats.github.io/tera/docs/)

## Template Features
Comes with a configured Github CI, Dependabot, server telemetry, and basic tests.

## To Do
> [!IMPORTANT]
> You'll need to download and install the following to begin working:
> - [Install and config](https://tailwindcss.com/blog/standalone-cli) the Tailwindcss cli.
> - Install [cargo-watch](https://crates.io/crates/cargo-watch): 
> `cargo install cargo-watch`
> - Install [just](https://github.com/casey/just#packages)

Make it your own:
- [ ] Update the package name in the `Cargo.toml`
- [ ] Update the `Dockerfile` with that same name
- [ ] Update the site name and description in `templates/base.html`
- [ ] Find and replace all instances in the source code of 'axum_tailwind_template' with your package name.
- [ ] Update this `README.md`!

## Developing
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

Also included in the `justfile` is:
```shell
just update
```
which will update dependancies and then run the tests.
