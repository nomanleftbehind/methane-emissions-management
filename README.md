# Emissions App

The aim of this project is to create a functional web app integrated with database and server for tracking and quantifiying greenhouse gas emissions.

## Technology

### Server
- [Rust](https://www.rust-lang.org/) programming language
- [Juniper](https://graphql-rust.github.io/) GraphQL server integration with [Actix](https://actix.rs/) web framework for Rust
- [Diesel](https://diesel.rs/) ORM to interact with PostgreSQL database

### Database
- [PostgreSQL](https://www.postgresql.org/) object-relational database

### Client
- [Svelte](https://svelte.dev/) web framework (not implemented yet)


I primarily started this project to learn Rust in practice. With that goal in mind, this documentation is written in a way to guide complete beginners through technology stack integration and towards successful compilation of the program.

## Setup

### Install Rust and Cargo
Follow the instructions on official [Rust](https://www.rust-lang.org/learn/get-started) website to install the language.

### Install PostgreSQL

Download and install [PostgreSQL](https://www.postgresql.org/download/). You will be asked to set up a password for superuser called "postgres" and port number and at the end of the installation.

Run `psql --port=5433 -U=postgres`. Make sure the port number is the same as the one you chose during the installation process.

You will be prompted to enter superuser password.

Run `CREATE DATABASE emissions;`.

### Install Diesel CLI
Make sure you have PostgreSQL in your Environment Variables Path. Example: `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin`.

Run `cargo install diesel_cli --no-default-features --features postgres` command in your terminal.

It's possible you will run into an error while installing. If that happens, follow official [Diesel](https://diesel.rs/guides/getting-started) guide on how to resolve it.

### Setup Diesel
We need to tell Diesel where to find our database. We do this by setting the DATABASE_URL environment variable.

Create .env file in the root folder of this project by running `echo DATABASE_URL=postgres://postgresql:superuserpassword@localhost:5433/emissions > .env` command.

Example of this file is `.env.example` and can be found in the root of this project.

The general form for a PostgreSQL connection URI is `postgresql://[user[:password]@][host][:port][/dbname]`

Populate your database by running `diesel migration run`. It executes `up.sql` commands located in `migrations` folder.

### Compile executable binary and run locally

Run `cargo run`

Open http://localhost:8080/playground, a GraphQL IDE for better development workflows, context-aware autocompletion & error highlighting.