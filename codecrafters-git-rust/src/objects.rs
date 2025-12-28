use std::io::BufReader;
use std::io::prelude::*;

use anyhow::Context;
use flate2::read::ZlibDecoder;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Blob,
    Tree,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Tree => write!(f, "tree"),
        }
    }
}

pub(crate) struct Object<R> {
    pub(crate) kind: Kind,
    pub(crate) expected_size: usize,
    pub(crate) reader: R,
}

impl Object<()> {
    pub(crate) fn read(hash: &str) -> anyhow::Result<Object<impl BufRead>> {
        let f = std::fs::File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .context("Error: Failed to read object file")?;

        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        let mut buf = Vec::new();
        z.read_until(0, &mut buf)
            .context("read from .git/object file")?;

        let header = String::from_utf8(buf).context("Converting bytes to string:")?;
        let header = header.trim_end_matches('\0');
        let (kind, size) = header.split_once(' ').context("Parsing object header")?;

        let kind = match kind {
            "blob" => Kind::Blob,
            "tree" => Kind::Tree,
            _ => anyhow::bail!("Unknown file type"),
        };
        let size = size
            .parse::<usize>()
            .context("Parsing size from &str to usize")?;

        Ok(Object {
            kind,
            expected_size: size,
            reader: z,
        })
    }
}
