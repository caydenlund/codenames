# Codenames

An implementation of the Codenames board game in Rust, where two teams take turns giving clues and guessing words.

The key feature of this application is that it is a single binary: the application combines a high-performance Actix-web backend with a modern Svelte frontend, compiled to a single binary for easy deployment.

## Architecture

### Backend

The backend is built using [Actix-web](https://actix.rs/), a powerful web framework for Rust.
It handles the server-side game state management, provides WebSocket connections for real-time gameplay, serves the compiled frontend assets, and leverages Rust's strict type safety for error checking.

### Frontend

The frontend is made with [Svelte](https://svelte.dev/), a modern JavaScript framework that compiles to highly efficient vanilla JS at build time.
It manages components and client-side game state management with Svelte stores.
Vite is the build tool providing a static webpage, with a `/index.html`, `/board/index.html`, etc. for embedding inside the backend.

## Usage

Run the server with

```bash
codenames-backend
```

### Command-line Arguments

- `-p`, `--port`: Set the port number (default: 8080)
- `--host`: Set the host address to bind to (default: 127.0.0.1)
- `--help`: Show all available options

### Environment Variables

- `RUST_LOG`: Set the logging level (`error`, `warn`, `info`, `debug`, `trace`)

### Development Environment

#### Frontend:

1. Navigate to the `frontend` directory
2. Install dependencies:
  ```bash
  npm install
  ```
3. Start the Vite development server:
  ```bash
  npm run dev
  ```

#### Backend:

1. Navigate to the `backend` directory
2. Use cargo as usual:
  ```bash
  cargo run
  ```

### Building for Production

1. Build the frontend:
  ```bash
  cd frontend
  npm run build
  ```
2. The Rust build process will then embed these files:
  ```bash
  cd backend
  cargo build --release
  ```

## License

Dual-licensed under either of the following, at your option:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
