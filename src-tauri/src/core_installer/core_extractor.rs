use std::io::Read;
use std::path::Path;

use crate::security::download_validator::DownloadValidator;
use crate::utils::{AppError, AppResult};

pub struct CoreExtractor;

impl CoreExtractor {
    /// Extract an archive to `dest`. The `format` parameter may be `"zip"` or
    /// `"tar.gz"`. An unsupported format returns `AppError::Unsupported`.
    pub fn extract(archive: &Path, dest: &Path, format: &str) -> AppResult<()> {
        match format {
            "zip" => Self::extract_zip(archive, dest),
            "tar.gz" | "tgz" => Self::extract_tar_gz(archive, dest),
            _ => Err(AppError::Unsupported(format!(
                "extract format '{format}' not supported"
            ))),
        }
    }

    /// Return the preferred archive format for the current platform:
    /// - Windows → `"zip"`
    /// - Linux / macOS → `"tar.gz"`
    pub fn platform_format() -> &'static str {
        if cfg!(target_os = "windows") {
            "zip"
        } else {
            "tar.gz"
        }
    }

    /// Extract a ZIP archive.
    fn extract_zip(archive: &Path, dest: &Path) -> AppResult<()> {
        let file =
            std::fs::File::open(archive).map_err(|e| AppError::Io(format!("open zip: {e}")))?;
        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| AppError::Io(format!("read zip: {e}")))?;
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| AppError::Io(format!("zip entry: {e}")))?;
            let out_path = dest.join(entry.name());

            // Prevent ZIP slip: validate the entry path does not escape dest
            DownloadValidator::check_zip_slip(entry.name(), dest)?;

            if entry.is_dir() {
                std::fs::create_dir_all(&out_path).ok();
                continue;
            }
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(|e| AppError::Io(format!("read zip entry: {e}")))?;
            std::fs::write(&out_path, &buf)
                .map_err(|e| AppError::Io(format!("write extracted: {e}")))?;
        }
        Ok(())
    }

    /// Extract a `.tar.gz` (or `.tgz`) archive using `flate2` + `tar`.
    fn extract_tar_gz(archive: &Path, dest: &Path) -> AppResult<()> {
        let file =
            std::fs::File::open(archive).map_err(|e| AppError::Io(format!("open tar.gz: {e}")))?;
        let decoder = flate2::read::GzDecoder::new(file);
        let mut ar = tar::Archive::new(decoder);
        ar.unpack(dest)
            .map_err(|e| AppError::Io(format!("extract tar.gz: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_zip() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("extract-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;

        let zip_path = tmp.join("test.zip");
        let file = std::fs::File::create(&zip_path)?;
        let mut zw = zip::ZipWriter::new(file);
        let opts = zip::write::SimpleFileOptions::default();
        zw.start_file("hello.txt", opts)
            .map_err(|e| AppError::Io(format!("zip: {e}")))?;
        use std::io::Write;
        zw.write_all(b"hello")
            .map_err(|e| AppError::Io(format!("zip write: {e}")))?;
        zw.finish()
            .map_err(|e| AppError::Io(format!("zip finish: {e}")))?;

        let dest = tmp.join("out");
        CoreExtractor::extract(&zip_path, &dest, "zip")?;
        assert!(dest.join("hello.txt").exists());
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn extracts_tar_gz() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("extract-tgz-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;

        let tgz_path = tmp.join("test.tar.gz");
        let file = std::fs::File::create(&tgz_path)?;

        // Build a tar.gz archive in memory
        let gz_encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut tar_builder = tar::Builder::new(gz_encoder);

        // Add a small header so tar doesn't choke on an empty archive
        let mut header = tar::Header::new_gnu();
        header.set_size(6);
        header.set_mode(0o644);
        header.set_cksum();
        tar_builder
            .append_data(&mut header, "hello.txt", &b"world\n"[..])
            .map_err(|e| AppError::Io(format!("tar append: {e}")))?;

        let gz = tar_builder
            .into_inner()
            .map_err(|e| AppError::Io(format!("tar finish: {e}")))?;
        gz.finish()
            .map_err(|e| AppError::Io(format!("gz finish: {e}")))?;

        let dest = tmp.join("out");
        CoreExtractor::extract(&tgz_path, &dest, "tar.gz")?;
        assert!(dest.join("hello.txt").exists());

        let content = std::fs::read_to_string(dest.join("hello.txt"))
            .map_err(|e| AppError::Io(format!("read: {e}")))?;
        assert_eq!(content, "world\n");
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn unsupported_format_returns_error() {
        let tmp = std::env::temp_dir().join("noop");
        let result = CoreExtractor::extract(&tmp, &tmp, "rar");
        assert!(result.is_err());
    }

    #[test]
    fn platform_format_is_known() {
        let fmt = CoreExtractor::platform_format();
        assert!(fmt == "zip" || fmt == "tar.gz");
    }
}
