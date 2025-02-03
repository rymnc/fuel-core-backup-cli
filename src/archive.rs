use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Cursor, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tar::{Archive, Builder, Header};

pub fn add_to_archive(src_dir: &Path, dest_file: &Path) -> anyhow::Result<()> {
    let file = File::create(dest_file)?;
    let writer = BufWriter::new(file);
    let mut tar = Builder::new(writer);

    fn collect_files(path: &Path, base_path: &Path) -> anyhow::Result<Vec<(PathBuf, PathBuf)>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(base_path)?.to_path_buf();

            if path.is_file() {
                files.push((relative_path, path));
            } else if path.is_dir() {
                files.extend(collect_files(&path, base_path)?);
            }
        }
        Ok(files)
    }

    let all_files = collect_files(src_dir, src_dir)?;

    // process the files in parallel w/ rayon
    let files = all_files
        .par_iter()
        .filter_map(|(relative_path, path)| {
            let metadata = path.metadata().ok()?;
            let file_content = File::open(path).ok()?;
            let mut header = Header::new_gnu();
            header.set_path(relative_path).ok()?;
            header.set_size(metadata.len());
            header.set_mode(metadata.permissions().mode());
            header.set_mtime(metadata.modified().ok()?.elapsed().ok()?.as_secs());
            header.set_cksum();

            let mut buffer = Vec::new();
            BufReader::new(file_content).read_to_end(&mut buffer).ok()?;
            Some((header, buffer))
        })
        .collect::<Vec<_>>();

    // tar needs sequential writes
    for (header, data) in files {
        tar.append(&header, Cursor::new(data))?;
    }

    tar.finish()?;
    Ok(())
}

pub fn extract_from_archive(src_file: &Path, dest_dir: &Path) -> anyhow::Result<()> {
    let src_file = Path::new(src_file);
    let dest_dir = Path::new(dest_dir);

    let file = File::open(src_file)?;
    let decoder = BufReader::new(file);
    let mut archive = Archive::new(decoder);

    archive.unpack(dest_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let src_dir = Path::new("src");
        let dest_file = Path::new("src.tar.xz");
        let dest_dir = Path::new("src_dir");

        add_to_archive(src_dir, dest_file).unwrap();
        extract_from_archive(dest_file, dest_dir).unwrap();

        assert!(dest_dir.exists());
        fs::remove_file(dest_file).unwrap();
        fs::remove_dir_all(dest_dir).unwrap();
    }
}
