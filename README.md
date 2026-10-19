# calory-cat-protocal

This README explains how to collaborate on this Gerust application.

<add a description of the project here>


## Prerequisites

* Rust (install via [`rustup`])
* [Docker]

## Project Structure

Distinct parts of the project are separated into separate crates:

```
.
├── cli    // CLI tools for running DB migrations or generating project files
├── config // Defines the `Config` struct and handles building the configuration from environment-specific TOML files and environment variables
├── db     // Encapsulates database access, migrations, as well as entity definitions and related code (if the project uses a database)
├── macros // Contains macros for application tests
└── web    // The web interface as well as tests for it
```

### Environment

The project uses _[.env](./.env)_ and _[.env.test](./.env.test)_ files to store configuration settings for the development and test environments respectively. Those files are read automatically by the parts of the application that require configuration settings to be present.

A Docker setup with preconfigured databases for the development and test environments is created out-of-the-box with the project. Boot the containers with:

```sh
docker compose up
```

The _.env_ and _.env.test_ contain matching configuration out-of-the-box.

## Commands

Running the application in development mode:

```sh
cargo run
```

Running the application tests:

```sh
cargo test
```

Running database tasks like executing migrations or seeding the database (see the [CLI crate](./cli/) for detailed documentation):

```sh
cargo db
```

Generating project files like entities, controllers, tests, etc. (see the [CLI crate](./cli/) for detailed documentation):

```sh
cargo generate
```

Building the project's docs:

## Building documentation

Build the project's documentation with:

```sh
cargo doc --workspace --all-features
```

[`rustup`]: https://rustup.rs
[docker]: https://www.docker.com
[web/middlewares/auth]: ./web/src/middlewares/auth.rs
[db/entities/tasks]: ./db/src/entities/tasks.rs
[web/controllers/tasks]: ./web/src/controllers/tasks.rs