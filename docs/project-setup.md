# Project Setup

## Clone the repository
Ensure that you have git installed, and run

```
git clone https://gitlab.com/openpowerlifting/opl-data.git
```

## Running the Makefile
In the `opl-data/` base directory, run

```make``` 

This will run the Makefile, building the project.

## [Optional] Building the Backend

Openpowerlifting is currently developing a backend in Rust using the Rocket web
framework.  To install this subproject, see the following steps.

### Installing Rust and Cargo

Visit [rustup](https://www.rustup.rs/) and download/run `rustup`, the Rust installer.

### Building the server
In the `server/` directory, run

```cargo build```

### Running the server
In the `server/` directory, run 

```cargo run```

The project should now be viewable at the default location of `localhost:8000`
