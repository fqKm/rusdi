
# 🚀 Rusdi Web Framework TODO List

## ✅ Stage 0: Project Setup
- [ ] Create new crate: `cargo new myfw --lib`
- [ ] Set up folder structure: server.rs, router.rs, request.rs, response.rs, handler.rs, middleware.rs
- [ ] Add dependencies in Cargo.toml: serde, serde_json, httparse

## 🔌 Stage 1: Core HTTP Server
- [ ] Wrap TcpListener in Server::new(host, port)
- [ ] Read and parse raw HTTP requests
- [ ] Build basic Request struct
- [ ] Build Response struct with .json() and .text()

## 🧭 Stage 2: Routing Engine
- [ ] Define Route struct with method, path, handler
- [ ] Implement route matching
- [ ] Create public API: app.route("/hello", "GET", handler_fn)

## 🧱 Stage 3: Handler Logic
- [ ] Define Handler type
- [ ] Support accessing req.body, headers, etc.
- [ ] Add req.json<T>() parser

## ⚙️ Stage 4: Middleware (Optional)
- [ ] Trait Middleware with handle(&mut Request) -> Result<(), Response>
- [ ] Register middlewares via app.middleware()

## 🗃️ Stage 5: Database Support
- [ ] Add support for Postgres/SQLite via sqlx or rusqlite
- [ ] Build AppState to share DB across handlers

## 🧪 Stage 6: Testing & Logging
- [ ] Add tests for router, request parsing, and response
- [ ] Add logging with env_logger

## 🌐 Stage 7: Public API Polishing
- [ ] Create macro for routing
- [ ] Ensure ergonomic user syntax: clean, minimal code

## 🚀 Stage 8: Crates.io Release
- [ ] Add Cargo.toml metadata
- [ ] Publish to crates.io
- [ ] Create examples/basic.rs
- [ ] Write README.md and documentation

## 💡 Stage 9+: Bonus Features
- [ ] Dynamic routing: /user/:id
- [ ] Template rendering
- [ ] WebSocket support
- [ ] Async (tokio)
- [ ] CLI scaffolding
- [ ] JWT middleware
