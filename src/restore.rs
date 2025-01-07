use fuel_core::combined_database::CombinedDatabase;

pub fn restore(restore_to: &str, backup_dir: &str) -> anyhow::Result<()> {
    let backup_dir = std::path::Path::new(backup_dir);
    let restore_to = std::path::Path::new(restore_to);
    CombinedDatabase::restore(restore_to, backup_dir)?;

    Ok(())
}