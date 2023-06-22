## Build & run

```bash
$ cargo build --release && cargo run --release
```

## Generate new migration & migrate

```bash
$ diesel migration generate create_users_table
$ diesel migration run
```