# Rust Starter MVC API

A reusable Rust REST API starter built with Axum, SeaORM, SQLx, and PostgreSQL. The project follows an MVC-inspired structure with separate routing, controller, service, model, and database entity layers.

## Technology Stack

- **Axum** — HTTP routing and request handling
- **Tokio** — Asynchronous runtime
- **SeaORM** — Database entities and ORM operations
- **SQLx** — PostgreSQL integration and database functionality
- **PostgreSQL** — Relational database
- **Serde** — JSON serialization and deserialization
- **dotenvy** — Environment variable loading
- **Argon2** — Password hashing

## Requirements

- Rust and Cargo
- PostgreSQL
- A configured database connection

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/rpmoulton/rust-starter-project.git
cd rust-starter-project
```

### 2. Configure environment variables

Create a `.env` file in the repository root.

Example:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/rust_starter
```

Adjust the database credentials and database name to match your local environment.

### 3. Install dependencies and compile

```bash
cargo build
```

### 4. Run database migrations

From the migration project directory:

```bash
cd migration
cargo run -- up
cd ..
```

### 5. Start the API

```bash
cargo run
```

## Project Structure

```text
.
├── src/
│   ├── controllers/    # HTTP request handlers
│   ├── entity/         # SeaORM database entities
│   ├── models/         # Application data structures
│   ├── routers/        # API route definitions
│   ├── server/         # Server configuration
│   ├── services/       # Business logic
│   └── main.rs         # Application entry point
├── migration/          # SeaORM migration project
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Development

Check compilation:

```bash
cargo check
```

Format code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features
```

Run tests:

```bash
cargo test
```

## Database Migrations

Database schema changes are managed using SeaORM migrations.

See [migration/README.md](migration/README.md) for migration commands.

## License

See the LICENSE file for licensing terms.
