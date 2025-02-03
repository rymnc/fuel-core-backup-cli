use fuel_core::combined_database::CombinedDatabase;

#[cfg(not(feature = "archive"))]
pub fn restore(restore_to: &str, backup_path: &str) -> anyhow::Result<()> {
    let backup_dir = std::path::Path::new(backup_path);
    let restore_to = std::path::Path::new(restore_to);
    CombinedDatabase::restore(restore_to, backup_dir)?;

    Ok(())
}

#[cfg(feature = "archive")]
pub fn restore(restore_to: &str, backup_path: &str) -> anyhow::Result<()> {
    use crate::archive::extract_from_archive;
    use tempfile::TempDir;

    let backup_path = std::path::Path::new(backup_path);
    let restore_to = std::path::Path::new(restore_to);
    let tmp_backup_dir = TempDir::new()?;

    extract_from_archive(backup_path, &tmp_backup_dir.path())?;
    CombinedDatabase::restore(restore_to, &tmp_backup_dir.path())?;

    Ok(())
}
