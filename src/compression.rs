use std::fs::{self, File};
use std::io::BufReader;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tar::{Archive, Builder, Header};
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

pub fn compress_directory(src_dir: &Path, dest_file: &Path) -> anyhow::Result<()> {
    let file = File::create(dest_file)?;
    // Wrap with an LZMA encoder
    let encoder = XzEncoder::new(file, 6);
    let mut tar = Builder::new(encoder);

    fn add_files_to_tar<P: AsRef<Path>>(
        tar: &mut Builder<XzEncoder<File>>,
        src_dir: P,
        base_path: &Path,
    ) -> anyhow::Result<()> {
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(base_path)?;

            if path.is_file() {
                let mut file = File::open(&path)?;
                let metadata = entry.metadata()?;
                let mut header = Header::new_gnu();

                header.set_path(relative_path)?;
                header.set_size(metadata.len());
                header.set_mode(metadata.permissions().mode());
                header.set_mtime(
                    metadata
                        .modified()?
                        .elapsed()
                        .map_err(|_| anyhow::anyhow!("Failed to get modified time of {:?}", path))?
                        .as_secs(),
                );
                header.set_cksum();

                tar.append(&header, &mut file)?;
            } else if path.is_dir() {
                add_files_to_tar(tar, &path, base_path)?;
            }
        }
        Ok(())
    }

    add_files_to_tar(&mut tar, src_dir, src_dir)?;

    // Finish writing the tar archive and flush LZMA encoder
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
