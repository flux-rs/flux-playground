# Flux Playground

Service to run flux in the browser.

## Development

For development run the server and client separately

### Run Server

To run the server

```bash
$ cargo run -- --flux-path /path/to/flux
```

This will start a server in port 3000.

Alternatively, you can install cargo watch:

```bash
$ cargo install cargo-watch
```

and run the server with

```bash
$ cargo watch -x "run -- --flux-path /path/to/flux"
```

### Run Client

**Install dependencies**

If you are running for the first time, install dependencies first

```bash
$ cd client
$ npm install
```

**Run vite server**

```bash
$ cd client
$ npm run dev
```

This will start a server for the client with hot reload. It prints the URL where it is served.

## Add examples

To add an example put a file in the [examples/](examples) directory and then add an appropriate entry to [config.yaml](examples/config.yaml). Files are checked as if they were inside the [`lib/`](./examples/lib/) directory, which means files in that directory can be declared as modules inside example files. See [`kmeans.rs`](./examples/kmeans.rs#L3) for an example that declares the `rvec` module.

## Deploy

### Build

First run

```bash
$ ./install.sh
```

This will build the server and client, and then move all necessary files to `./dist`.

### Start server

To start the server

```bash
$ cd dist
$ ./flux-playground --flux-path /path/to/flux
```
