# Methane Emissions Reduction Program

Open-source web application used for managing, quantifying, and reporting methane emissions in upstream oil and gas sector.

Methane is a potent greenhouse gas with at least 25 times the warming potential of carbon dioxide (CO2) over a 100-year period and scientists estimate that it is responsible for 30% of observed global warming to date. Therefore, cutting methane emissions is the most cost-effective way to reduce greenhouse gas emissions. Oil and gas sector is the largest source of methane emissions in US and Canada, accounting for 30% and 41% respectively in 2021.

Methane emission reduction efforts from petroleum and natural gas sector are underway in both countries. In US, EPA has introduced Methane Emissions Reduction Program, while in Canada's biggest oil and gas producing province Alberta, local energy regulator AER amended [Directive 060: Upstream Petroleum Industry Flaring, Incinerating, and Venting](https://www.aer.ca/regulating-development/rules-and-directives/directives/directive-060) and [Manual 015: Estimating Methane Emissions](https://static.aer.ca/prd/documents/manuals/Manual015.pdf) to backstop methane emissions target.


This project was primarily started as a side hustle to learn Rust programming language in practice. Over time, as it grew in practical usefulness and, most importantly, became more compatible with specifications set out in AER's Directive 060 and Manual 015, it became a serious endeavour to develop an open-source tool to be used by those tasked with managing, quantifying, and reporting methane emissions.

## Technology

### Server
- [Rust](https://www.rust-lang.org/) programming language
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/index.html) server integration with [Actix](https://actix.rs/) web framework for Rust
- [SQLx](https://github.com/launchbadge/sqlx) an async, pure Rust SQL crate

### Database
- [PostgreSQL](https://www.postgresql.org/) object-relational database

### Client
- [Yew](https://yew.rs/) a Rust framework for creating multi-threaded front-end web apps with WebAssembly

This documentation is written in a way to guide complete beginners through technology stack integration and towards successful compilation of the program.

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
`./build-debug.sh`

On Windows run:
```PowerShell.exe -File build-debug.ps1 -ExecutionPolicy Bypass```

Open http://localhost:8081/ in your browser to view the app.

You can also test your queries and mutations in GraphQL Playground by opening http://localhost:8081/graphiql in you browser.


## Screenshots

![screenshot](/docs/app.png?raw=true)

</ul>


<h1>Application Model</h1>
<table>
    <colgroup>
        <col>
        <col span="16" class="pneumatic-devices">
        <col span="6" class="compressor-seals">
        <col span="7" class="defined-vent-gas">
    </colgroup>
    <tr>
        <td> </td>
        <th scope="col" colspan="16">Pneumatic Devices</th>
        <th scope="col" colspan="6">Compressor Seals</th>
        <th scope="col" colspan="7">Defined Vent Gas</th>
    </tr>
    <tr>
        <th scope="row">Root</th>
        <td colspan="29" style="text-align: center;">Users</td>
    </tr>
    <tr>
        <th scope="row">Reporting Level</th>
        <td colspan="29" style="text-align: center;">Facilities</td>
    </tr>
    <tr>
        <th scope="row">Surface Lease Level</th>
        <td colspan="29" style="text-align: center;">Sites</td>
    </tr>
    <tr>
        <th scope="row">Emitter Level</th>
        <td colspan="5" style="text-align: center;">Pneumatic Instruments</td>
        <td colspan="6" style="text-align: center;">Level Controllers</td>
        <td colspan="5" style="text-align: center;">Pneumatic Pumps</td>
        <td colspan="6" style="text-align: center;">Compressor Seals</td>
        <td colspan="7" style="text-align: center;">Storage Tanks</td>
    </tr>
    <tr>
        <th scope="row">Calculation Level</th>
        <td colspan="1" style="text-align: center;">Emission Rates</td>
        <td colspan="1" style="text-align: center;">Controlled Characterizations</td>
        <td colspan="1" style="text-align: center;">Control Device Inactivity</td>
        <td colspan="1" style="text-align: center;">Month Hours</td>
        <td colspan="1" style="text-align: center;">Overrides</td>
        <td colspan="1" style="text-align: center;">Emission Rates</td>
        <td colspan="1" style="text-align: center;">Actuation Frequencies</td>
        <td colspan="1" style="text-align: center;">Controlled Characterizations</td>
        <td colspan="1" style="text-align: center;">Control Device Inactivity</td>
        <td colspan="1" style="text-align: center;">Month Hours</td>
        <td colspan="1" style="text-align: center;">Overrides</td>
        <td colspan="1" style="text-align: center;">Emission Rates</td>
        <td colspan="1" style="text-align: center;">Controlled Characterizations</td>
        <td colspan="1" style="text-align: center;">Control Device Inactivity</td>
        <td colspan="1" style="text-align: center;">Month Hours</td>
        <td colspan="1" style="text-align: center;">Overrides</td>
        <td colspan="1" style="text-align: center;">Tests</td>
        <td colspan="1" style="text-align: center;">Controlled Characterizations</td>
        <td colspan="1" style="text-align: center;">Control Device Inactivity</td>
        <td colspan="1" style="text-align: center;">Emission Surveys</td>
        <td colspan="1" style="text-align: center;">Month Hours</td>
        <td colspan="1" style="text-align: center;">Overrides</td>
        <td colspan="1" style="text-align: center;">Changes</td>
        <td colspan="1" style="text-align: center;">GIS Factors</td>
        <td colspan="1" style="text-align: center;">Liquid Hydrocarbon Entering</td>
        <td colspan="1" style="text-align: center;">Controlled Characterizations</td>
        <td colspan="1" style="text-align: center;">Control Device Inactivity</td>
        <td colspan="1" style="text-align: center;">Emission Surveys</td>
        <td colspan="1" style="text-align: center;">Overrides</td>
    </tr>
    <tr>
        <th scope="row">Quantification Level</th>
        <td colspan="29" style="text-align: center;">Month Methane Emissions</td>
    </tr>
</table>