use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufReader, Cursor, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tar::{Archive, Builder, Header};
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

pub fn compress_directory(src_dir: &Path, dest_file: &Path) -> anyhow::Result<()> {
    let file = File::create(dest_file)?;
    let encoder = XzEncoder::new(file, 6);
    let mut tar = Builder::new(encoder);
    let entries: Vec<_> = fs::read_dir(src_dir)?.filter_map(Result::ok).collect();
    let base_path = src_dir.to_path_buf();

    // use par_iter from rayon
    // we don't care about sorting these files
    let files = entries
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let relative_path = path.strip_prefix(&base_path).ok()?.to_path_buf();
            let metadata = entry.metadata().ok()?;

            if path.is_file() {
                let file_content = File::open(&path).ok()?;
                let mut header = Header::new_gnu();
                header.set_path(&relative_path).ok()?;
                header.set_size(metadata.len());
                header.set_mode(metadata.permissions().mode());
                header.set_mtime(metadata.modified().ok()?.elapsed().ok()?.as_secs());
                header.set_cksum();

                let mut buffer = Vec::new();
                BufReader::new(file_content).read_to_end(&mut buffer).ok()?;
                Some((header, buffer))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // tar needs sequential writes
    for (header, data) in files {
        tar.append(&header, Cursor::new(data))?;
    }

    tar.finish()?;
    Ok(())
}

pub fn decompress_archive(src_file: &Path, dest_dir: &Path) -> anyhow::Result<()> {
    let src_file = Path::new(src_file);
    let dest_dir = Path::new(dest_dir);

    let file = File::open(src_file)?;
    let decoder = XzDecoder::new(BufReader::new(file));
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

        compress_directory(src_dir, dest_file).unwrap();
        decompress_archive(dest_file, dest_dir).unwrap();

        assert!(dest_dir.exists());
        fs::remove_file(dest_file).unwrap();
        fs::remove_dir_all(dest_dir).unwrap();
    }
}
