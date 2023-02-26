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


<style>
    .container {
        display: grid;
        border: 1px solid red;
        grid-template-rows: auto;
        grid-template-columns: repeat(3, 1fr);
        row-gap: 20px;
        column-gap: 50px;
        height: auto;
        width: fit-content;
        align-items: center;
        justify-items: center;
        position: relative;
    }
    .nodetest {
        border: 1px solid white;
    }
    .node {
        width: 90px;
        height: 90px;
        border-radius: 50%;
        border: 2px solid yellow;
        text-align: center;
        line-height: 90px;
        position: relative;
        background-color: green;
    }
    .text {
        display: inline-block;
        vertical-align: middle;
        line-height: 15px;
        word-wrap: break-word;
    }
    .node-small {
        width: 45px;
        height: 45px;
        font-size: 60%;
        border-radius: 50%;
        border: 1px solid yellow;
        text-align: center;
        line-height: 45px;
        position: relative;
        background-color: green;
    }
</style>
<h1>Application Model</h1>
<div class="container">
    <div style="grid-column: 1; grid-row: 1;"><span class="text">Root</span></div>
    <div style="grid-column: 1; grid-row: 2;"><span class="text">Reporting level</span></div>
    <div style="grid-column: 1; grid-row: 3;"><span class="text">Emitters</span></div>
    <div style="grid-column: 1; grid-row: 4;"><span class="text">Calculation Level</span></div>
    <div style="grid-column: 1; grid-row: 5;"><span class="text">Volumes</span></div>
    <div class="node" style="grid-column: 3; grid-row: 1;"><span class="text">Users</span></div>
    <div class="node" style="grid-column: 3; grid-row: 2;"><span class="text">Facilities</span></div>
    <div style="display: grid; grid-column: 2; grid-row: 3;">
        <div class="node" style="grid-column: 1; grid-row: 1/3;"><span class="text">Controllers</span></div>
        <div class="node-small" style="grid-column: 2; grid-row: 1;"><span class="text">Controller Application</span>
        </div>
        <div class="node-small" style="grid-column: 2; grid-row: 2;"><span class="text">Controller Manufacturer</span>
        </div>
    </div>
    <div style="display: grid; grid-column: 2; grid-row: 4;">
        <div class="node" style="grid-column: 1; grid-row: 1;"><span class="text">Controller Changes</span></div>
        <div class="node" style="grid-column: 2; grid-row: 1;"><span class="text">Controller Month Hours</span></div>
    </div>
    <div class="node" style="grid-column: 3; grid-row: 3;"><span class="text">Compressors</span></div>
    <div style="display: grid; grid-column: 3; grid-row: 4;">
        <div class="node" style="grid-column: 1; grid-row: 1;"><span class="text">Compressor Changes</span></div>
        <div class="node" style="grid-column: 2; grid-row: 1;"><span class="text">Compressor Blowdowns</span></div>
        <div class="node" style="grid-column: 3; grid-row: 1;"><span class="text">Compressor Month Hours</span></div>
    </div>
    <div class="node" style="grid-column: 4; grid-row: 3;"><span class="text">Tank Farms</span></div>
    <div style="display: grid; grid-column: 4; grid-row: 4;">
        <div class="node" style="grid-column: 1; grid-row: 1;"><span class="text">Tank Farm Changes</span></div>
        <div class="node" style="grid-column: 2; grid-row: 1;"><span class="text">Tank Farm Vent Factor</span></div>
        <div class="node" style="grid-column: 13; grid-row: 1;"><span class="text">Tank Farm Month Oil Flow</span></div>
    </div>
    <div class="node" style="grid-column: 2; grid-row: 5"><span class="text">Controllers Monthly Vent</span></div>
    <div class="node" style="grid-column: 3; grid-row: 5"><span class="text">Compressors Monthly Vent</span></div>
    <div class="node" style="grid-column: 4; grid-row: 5"><span class="text">Tank Farms Monthly Vent</span></div>
</div>