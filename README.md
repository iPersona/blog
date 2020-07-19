# About

This is my personal blog.

## Architecture

![img](imges/architecture.png)

## Dependences

- Redis
- Postgresql

## Getting Started

### [Rust](https://www.rust-lang.org/)

```bash
curl https://sh.rustup.rs -sSf | sh
```

### [Diesel Cli](https://github.com/diesel-rs/diesel)

This project use Diesel as Orm framework, so you need to install its command line tool via Rust package manager(eg, Cargo)

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### [Postgresql](https://www.postgresql.org/)

you need to install Postgresql database, and then configure postgresql by following document’s guide

#### Install the corresponding version of contrib

```bash
# Fedora
yum install postgresql96-contrib
# For Ubuntu
sudo apt-get install libpq-dev
```

#### init database

```bash
./init.sh # press 3 to init database
diesel migration run # this will renew schema.rs without view created in create_tags/up.sql
```

[Diesel.rs Trick: Treat View as Table](https://deterministic.space/diesel-view-table-trick.html)

### blog

```bash
cargo run  
```

if you want to login admin, the account is `admin`, password is `admin`

## Other documents

- [API](docs/api.md)
- [Docker](docs/docker.md)
- [Postgres](docs/pg.md)

## Q&A

### Could not compile `awc`

```bash
cargo clean
rm cargo.lock
cargo build
```

### the trait `diesel::Queryable<diesel::sql_types::Text, diesel::pg::Pg>` is not implemented for `std::option::Option<std::string::String>`

#### REASON

the field order of the struct defined in rust code is not the same in schema.rs

#### HOW TO FIX

make the filed order the same in schema.rs

### view is NOT generated as table in `schema.rs` with `diesel migration run`

#### HOW TO FIX

use init.sh script under docker folder to automatically add the view as table into`schema.rs`

## Useful Documents

- [Postgres & Diesel types](https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html)
- [How to implement Email verify](https://segmentfault.com/a/1190000014522351)

### the trait bound `diesel::query_builder::SqlQuery: diesel::query_dsl::LoadQuery<_, models::comment::Comments>` is not satisfied

#### REASON

The definition of the structure is not the same as which defined in the `schema.rs`

#### HOW TO FIX

Find and change the incorrect field type to that defined in `schema.rs`

## TODO
