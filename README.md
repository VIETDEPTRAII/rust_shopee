# Shopee Rust API

A Rust-based REST API using Actix-web framework with MySQL database.

## Prerequisites

- Docker
- Docker Compose

## Project Structure

```
shopee/
├── src/
│   └── models/
│       ├── mod.rs
│       └── shop.rs
├── migrations/
│   └── 20231130000000_create_shop_table.sql
├── Dockerfile
├── docker-compose.yml
├── Cargo.toml
└── .env
```

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd shopee
```

2. Start the application:
```bash
docker-compose up --build
```

This will:
- Build the Rust application
- Start MySQL database
- Install necessary tools (cargo-watch, sqlx-cli)
- Start the development server with hot reload

The API will be available at `http://localhost:8080`

## Database Migrations

The project uses SQLx for database migrations. Migration files are located in the `migrations/` directory.

### Running Migrations

1. Run migrations inside Docker container:
```bash
# Execute migrations
docker-compose exec app sqlx migrate run

# Revert last migration
docker-compose exec app sqlx migrate revert
```

2. Create a new migration:
```bash
docker-compose exec app sqlx migrate add <migration_name>
```

This will create a new file in the `migrations/` directory.

## Development

### Hot Reload

The application uses `cargo-watch` for hot reloading. Any changes to the source code will automatically trigger a rebuild.

### Environment Variables

The following environment variables can be configured in `docker-compose.yml`:

```yaml
environment:
  DATABASE_URL: mysql://user:password@db:3306/actix_db?connect_timeout=60
  RUST_LOG: debug
  RUST_BACKTRACE: 1
```

### Database Configuration

MySQL database settings can be modified in `docker-compose.yml`:

```yaml
MYSQL_ROOT_PASSWORD: root_password
MYSQL_USER: user
MYSQL_PASSWORD: password
MYSQL_DATABASE: actix_db
```

## Project Components

- **Web Framework**: Actix-web 4.4
- **Database**: MySQL 8.0
- **ORM**: SQLx 0.7
- **Development Tools**:
  - cargo-watch (hot reload)
  - sqlx-cli (database migrations)

## Docker Volumes

The project uses three Docker volumes:
- `db_data`: Persists MySQL database data
- `cargo-cache`: Caches Rust dependencies
- `target-cache`: Caches compiled artifacts

## Troubleshooting

1. If the database connection fails:
   - Ensure MySQL container is running: `docker-compose ps`
   - Check database logs: `docker-compose logs db`

2. If migrations fail:
   - Verify DATABASE_URL is correct
   - Check migration files in `migrations/` directory
   - Review database logs: `docker-compose logs db`

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request
# rust_shopee
