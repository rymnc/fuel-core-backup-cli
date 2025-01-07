use fuel_core::combined_database::CombinedDatabase;

pub fn backup(db_dir: &str, backup_dir: &str) -> anyhow::Result<()> {
    let backup_dir = std::path::Path::new(backup_dir);
    let db_dir = std::path::Path::new(db_dir);
    CombinedDatabase::backup(db_dir, backup_dir)?;

    Ok(())
}