# fuel-core-backup-cli

This is a CLI tool to backup and restore Fuel Core databases (rocksdb).

The tests for backup and restore are already located in [fuel-core](https://github.com/fuellabs/fuel-core).

## Usage

```bash
cargo run -- --help
```


## Backup (without compression)

```bash
cargo run --release -- backup --backup-from /path/to/db --backup-to /path/to/backup
```

## Backup (with compression)

```bash
cargo run --release --features compress -- backup --backup-from /path/to/db --backup-to /path/to/backup.xz
```

## Restore (without compression)

```bash
cargo run --release -- restore --restore-from /path/to/backup --restore-to /path/to/db
```

## Restore (with compression)

```bash
cargo run --release --features compress -- restore --restore-from /path/to/backup.xz --restore-to /path/to/db
```



