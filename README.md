# fuel-core-backup-cli

This is a CLI tool to backup and restore Fuel Core databases (rocksdb).

The tests for backup and restore are already located in [fuel-core](https://github.com/fuellabs/fuel-core)

## Usage

```bash
cargo run -- --help
```


## Backup

```bash
cargo run -- backup --backup-from /path/to/db --backup-to /path/to/backup
```

## Restore

```bash
cargo run -- restore --restore-from /path/to/backup --restore-to /path/to/db
```

