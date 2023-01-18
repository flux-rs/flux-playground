# Flux Playground

Service to run flux in the browser.

## Dev

### Run Server

To run the server

```bash
cargo run -- --rustc-flux-path /path/to/rustc-flux
```

Alternatively you can install cargo watch:

```bash
cargo install cargo-watch
```

and run server with

```bash
cargo watch -x "run -- --rustc-flux-path /path/to-/rustc-flux"
```

This will start a server in port 3000.

### Run Client

```bash
cd client
npm run dev
```

This will start a server for the client with auto-reload. It prints the URL where it is served.

## Deploy

The script `[install.sh]` will build the server and the client and put all neccesary fields in
the directory `dist`. Then to run the server.

```bash
cd dist
./flux-playground --rustc-flux-path /path/to/rustc-flux
```
