# Emissions App

The aim of this project is to create a web app integrated with database and server, fully implemented using Rust language stack. App is used for tracking and quantifiying greenhouse gas emissions.

## Technology

### Server
- [Rust](https://www.rust-lang.org/) programming language
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/index.html) server integration with [Actix](https://actix.rs/) web framework for Rust
- [Diesel](https://diesel.rs/) ORM to interact with PostgreSQL database

### Database
- [PostgreSQL](https://www.postgresql.org/) object-relational database

### Client
- [Yew](https://yew.rs/) Rust framework for creating multi-threaded front-end web apps using WebAssembly


I primarily started this project to learn Rust in practice. With that goal in mind, this documentation is written in a way to guide complete beginners through technology stack integration and towards successful compilation of the program.

Big shout-out to [philipdaquin](https://github.com/philipdaquin/Twitter-Clone-WASM), [open-msupply](https://github.com/openmsupply/open-msupply), and [zzy](https://github.com/zzy/tide-async-graphql-mongodb) without whom this process would have taken a lot longer.

## Setup

### Backend

- #### Install Rust and Cargo
<ul>

Follow the instructions on official [Rust](https://www.rust-lang.org/learn/get-started) website to install the language.
</ul>

- #### Install PostgreSQL

<ul>

Download and install [PostgreSQL](https://www.postgresql.org/download/). You will be asked to set up a password for superuser called "postgres" and port number at the end of the installation.

Run `psql --port=5432 -U=postgres`. Make sure the port number is the same as the one you chose during the installation process.

You will be prompted to enter superuser password.

Run `CREATE DATABASE emissions;` and `CREATE DATABASE emissions_test;`.</ul>

- #### Install Diesel CLI
<ul>

Make sure you have PostgreSQL in your Environment Variables Path. Example: `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin`.

Run `cargo install diesel_cli --no-default-features --features postgres` command in your terminal.

It's possible you will run into an error while installing. If that happens, follow official [Diesel](https://diesel.rs/guides/getting-started) guide on how to resolve it.</ul>

- #### Setup Diesel
<ul>We need to tell Diesel where to find our database. We do this by setting the DATABASE_URL environment variable.

Rename `backend/.env.example` file to `backend/.env`. Inside, replace the word 'password' of DATABASE_URL with earlier defined superuser password.

The general form for a PostgreSQL connection URI is `postgresql://[user[:password]@][host][:port][/dbname]`

Navigate to `backend/src/repository` folder in your terminal and run `diesel migration run` to populate your database. It executes `up.sql` files located in `backend/src/repository/migrations` folder.</ul>

- #### Build binary and run locally
<ul>

Navigate back to backend root folder `./backend/`  Compile program by running `cargo run`.
</ul>

- #### Potential errors during build
<ul>

If you are compiling on Windows machine you might run into an error with `argonautica` package as it requires `libclang.dll`, which is part of LLVM. Download [LLVM](https://github.com/llvm/llvm-project/releases/tag/llvmorg-14.0.0). Click `LLVM-14.0.0-win64.exe` or `LLVM-14.0.0-win32.exe`, depending on your system. During installation make sure you select to add LLVM to your path. Compilation should be succesful after installing it. You might run into `error: failed to run custom build command for rdkafka-sys`. Installing the latest version of [CMake](https://cmake.org/download/) should solve that.</ul>

Navigate to http://localhost:8081/graphiql in your browser. It opens a GraphQL server IDE where you can test your queries and mutations. It features context-aware autocompletion and error highlighting.

### Frontend

Install [Trunk](https://trunkrs.dev/), a WASM web application bundler for Rust, and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/), a Rust library and CLI tool that facilitate high-level interactions between WASM modules and JavaScript by running `cargo install trunk wasm-bindgen-cli`.

Navigate to `./frontend/` folder in your terminal and run `trunk build`.

Run `trunk serve` to start the web app and open http://localhost:4001 in your browser.