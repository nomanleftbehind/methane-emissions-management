# Emissions App

The aim of this project is to create a web app for tracking and quantifiying greenhouse gas emissions.

Server is implemented using Rust language stack for GraphQL, and Client using Svelte.

## Technology

### Server
- [Rust](https://www.rust-lang.org/) programming language
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/index.html) server integration with [Actix](https://actix.rs/) web framework for Rust
- [SQLx](https://github.com/launchbadge/sqlx) an async, pure Rust SQL crate

### Database
- [PostgreSQL](https://www.postgresql.org/) object-relational database

### Client
- [Svelte](https://svelte.dev/) compiler for building web applications


I primarily started this project to learn Rust in practice. With that goal in mind, this documentation is written in a way to guide complete beginners through technology stack integration and towards successful compilation of the program.

Big shout-out to [open-msupply](https://github.com/openmsupply/open-msupply), and [Matt Wilkinson](https://github.com/AU-Health/au-health-backend) without whom this process would have taken a lot longer.

## Setup

### Backend

- ### Install Rust and Cargo
<ul>

Follow the instructions on official [Rust](https://www.rust-lang.org/learn/get-started) website to install the language.
</ul>

- ### Install PostgreSQL

<ul>

Download and install [PostgreSQL](https://www.postgresql.org/download/). You will be asked to set up a password for superuser called "postgres" and port number at the end of the installation.

</ul>

- ### Install SQLx CLI
<ul>

`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

</ul>

- ### Setup SQLx
<ul>
We need to tell SQLx where to find our database. We do this by setting the DATABASE_URL environment variable.

Rename `.env.example` file to `.env`. Inside, replace the word 'password' of DATABASE_URL with earlier defined superuser password.

The general form for a PostgreSQL connection URI is `postgresql://[user[:password]@][host][:port][/dbname]`

Run `sqlx database create`

Create database tables by running `sqlx migrate run`.

</ul>

- ### Build binary and run locally
<ul>

`cargo run`
</ul>


### Frontend

Navigate to client folder with `cd client`.

Run `npm install` to install frontend dependencies.

Run `npm run generate` to generate queries, mutations and types defined on the server for frontend consumption.

Run `npm run dev` to start the web app and open http://localhost:5173/ in your browser.