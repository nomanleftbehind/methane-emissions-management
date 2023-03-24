# Emissions App

The aim of this project is to create a web app for tracking and quantifiying greenhouse gas emissions.

Server is implemented using Rust language stack for GraphQL, and Client is Rust compiled to WebAssembly.

## Technology

### Server
- [Rust](https://www.rust-lang.org/) programming language
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/index.html) server integration with [Actix](https://actix.rs/) web framework for Rust
- [SQLx](https://github.com/launchbadge/sqlx) an async, pure Rust SQL crate

### Database
- [PostgreSQL](https://www.postgresql.org/) object-relational database

### Client
- [Yew](https://yew.rs/) a Rust framework for creating multi-threaded front-end web apps with WebAssembly


I primarily started this project to learn Rust in practice. With that goal in mind, this documentation is written in a way to guide complete beginners through technology stack integration and towards successful compilation of the program.

Big shout-out to [open-msupply](https://github.com/openmsupply/open-msupply), and [Matt Wilkinson](https://github.com/AU-Health/au-health-backend) without whom this process would have taken a lot longer.

## Setup

### Backend

- ### Install Rust and Cargo
<ul>

Follow the instructions on official [Rust](https://www.rust-lang.org/learn/get-started) website to install the language.

</ul>

- ### Install WASM and Trunk
  
<ul>

`rustup target add wasm32-unknown-unknown`

`cargo install --locked trunk`

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

Client is rendered on server side so no need to serve with Trunk.

Following command will produce index.html, along with the compiled css, WASM & the JS loader for the WASM, and it will run server in debug mode.

On Mac run:
`.\build-debug.sh`

On Windows run:
```PowerShell.exe -File build-debug.ps1 -ExecutionPolicy Bypass```

Open http://localhost:8001/ in your browser to view the app.

You can also test your queries and mutations in GraphQL Playground by opening http://localhost:8081/graphiql in you browser.

</ul>


<h1>Application Model</h1>
<table>
    <colgroup>
        <col>
        <col span="2" class="controllers">
        <col span="3" class="compressors">
        <col span="3" class="tank-farms">
    </colgroup>
    <tr>
        <td> </td>
        <th scope="col" colspan="2">Controllers</th>
        <th scope="col" colspan="3">Compressors</th>
        <th scope="col" colspan="3">Tank Farms</th>
    </tr>
    <tr>
        <th scope="row">Root</th>
        <td colspan="8" style="text-align: center;">Users</td>
    </tr>
    <tr>
        <th scope="row">Reporting level</th>
        <td colspan="8" style="text-align: center;">Facilities</td>
    </tr>
    <tr>
        <th scope="row">Emitters</th>
        <td colspan="1" style="text-align: center;">Controllers</td>
        <td colspan="1">
            <table>
                <tr><td style="text-align: center;">Controller Application</td></tr>
                <tr><td style="text-align: center;">Controller Manufacturer</td></tr>
            </table>
        </td>
        <td colspan="3" style="text-align: center;">Compressors</td>
        <td colspan="3" style="text-align: center;">Tank Farms</td>
    </tr>
    <tr>
        <th scope="row">Calculation Level</th>
        <td colspan="1" style="text-align: center;">Controller Changes</td>
        <td colspan="1" style="text-align: center;">Controller Month Hours</td>
        <td colspan="1" style="text-align: center;">Compressor Changes</td>
        <td colspan="1" style="text-align: center;">Compressor Blowdowns</td>
        <td colspan="1" style="text-align: center;">Compressor Month Hours</td>
        <td colspan="1" style="text-align: center;">Tank Farm Changes</td>
        <td colspan="1" style="text-align: center;">Tank Farm Vent Factor</td>
        <td colspan="1" style="text-align: center;">Tank Farm Month Oil Flow</td>
    </tr>
    <tr>
        <th scope="row">Volumes</th>
        <td colspan="2" style="text-align: center;">Controllers Monthly Vent</td>
        <td colspan="3" style="text-align: center;">Compressors Monthly Vent</td>
        <td colspan="3" style="text-align: center;">Tank Farms Monthly Vent</td>
    </tr>
</table>