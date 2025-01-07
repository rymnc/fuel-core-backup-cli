use fuel_core::combined_database::CombinedDatabase;

#[cfg(not(feature = "compress"))]
pub fn backup(db_dir: &str, backup_path: &str) -> anyhow::Result<()> {
    let backup_dir = std::path::Path::new(backup_path);
    let db_dir = std::path::Path::new(db_dir);
    CombinedDatabase::backup(db_dir, backup_dir)?;

    Ok(())
}

#[cfg(feature = "compress")]
pub fn backup(db_dir: &str, backup_path: &str) -> anyhow::Result<()> {
    use crate::compression::compress_directory;

    let tmp_backup_dir = std::env::temp_dir().join("fuel-db-backup");
    let db_dir = std::path::Path::new(db_dir);
    let backup_file = std::path::Path::new(backup_path);

    CombinedDatabase::backup(db_dir, &tmp_backup_dir)?;
    compress_directory(&tmp_backup_dir, backup_file)?;
    std::fs::remove_dir_all(&tmp_backup_dir)?;

    Ok(())
}
