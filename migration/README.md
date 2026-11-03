# Database Migrations

This project uses SeaORM's migration framework to manage PostgreSQL schema changes.

All commands below should be run from the `migration/` directory.

## Creating migrations

Generate a new migration:

```bash
cargo run -- generate MIGRATION_NAME
```

Implement the generated migration, then apply it to the database.

## Applying migrations

Apply all pending migrations:

```bash
cargo run -- up
```

Apply a specific number of pending migrations:

```bash
cargo run -- up -n 10
```

## Rolling back migrations

Rollback the most recently applied migration:

```bash
cargo run -- down
```

Rollback multiple migrations:

```bash
cargo run -- down -n 10
```

## Checking migration status

```bash
cargo run -- status
```

## Resetting the database

**Warning:** The following commands may permanently delete database data. Use them only when you understand their effects.

Drop all tables and reapply migrations:

```bash
cargo run -- fresh
```

Rollback all migrations and reapply them:

```bash
cargo run -- refresh
```

Rollback all applied migrations:

```bash
cargo run -- reset
```

## Configuration

Ensure that your PostgreSQL connection is configured using the `DATABASE_URL` environment variable before running migrations.
