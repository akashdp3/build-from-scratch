use anyhow::Context;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;

pub(crate) fn invoke(write: bool, file: String) -> anyhow::Result<()> {
    // main content
    let content = fs::read(file).context("Failed to read content from file")?;
    let header = format!("blob {}\0", content.len()).into_bytes();
    let result = [&header[..], &content[..]].concat();

    // encoder
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(&result).unwrap();
    let compressed_bytes = encoder.finish().unwrap();

    let mut hasher = Sha1::new();
    hasher.update(&result);
    let hash = hasher.finalize();
    let git_hash = format!("{:x}", hash);

    if write {
        // Create object directory and file
        fs::create_dir(format!(".git/objects/{}", &git_hash[0..2]))
            .context("Failed to create dir")?;
        fs::write(
            format!(".git/objects/{}/{}", &git_hash[0..2], &git_hash[2..]),
            compressed_bytes,
        )
        .context("Failed to create file")?;

        print!("{}", git_hash);
    }
    Ok(())
}
