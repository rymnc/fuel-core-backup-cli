use fuel_core::combined_database::CombinedDatabase;

#[cfg(not(feature = "archive"))]
pub fn backup(db_dir: &str, backup_path: &str) -> anyhow::Result<()> {
    let backup_dir = std::path::Path::new(backup_path);
    let db_dir = std::path::Path::new(db_dir);
    CombinedDatabase::backup(db_dir, backup_dir)?;

    Ok(())
}

#[cfg(feature = "archive")]
pub fn backup(db_dir: &str, backup_path: &str) -> anyhow::Result<()> {
    use crate::archive::add_to_archive;
    use tempfile::TempDir;

    let tmp_backup_dir = TempDir::new()?;
    let db_dir = std::path::Path::new(db_dir);
    let backup_file = std::path::Path::new(backup_path);

    CombinedDatabase::backup(db_dir, &tmp_backup_dir.path())?;
    add_to_archive(&tmp_backup_dir.path(), backup_file)?;

    Ok(())
}
