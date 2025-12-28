use crate::objects::{Kind, Object};
use anyhow::Context;
use std::io::prelude::*;
use std::io::{self};

pub(crate) fn invoke(name_only: bool, object_hash: &str) -> anyhow::Result<()> {
    let mut object = Object::read(object_hash)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut buf = Vec::new();

    loop {
        buf.clear();
        object
            .reader
            .read_until(0, &mut buf)
            .context("read from .git/object file")?;

        if buf.is_empty() {
            break;
        }

        let buf_str = String::from_utf8(buf.clone()).context("Parse utf8 into str")?;
        let buf_str = buf_str.trim_end_matches("\0");
        let (mode, dir_name) = buf_str.split_once(' ').context("Spliting buf_str")?;

        let mut hash_bytes = [0u8; 20];
        object
            .reader
            .read_exact(&mut hash_bytes)
            .context("reading hash")?;

        if name_only {
            writeln!(handle, "{}", dir_name).context("Failed to write dir_name to stdout")?;
        } else {
            let hash_str = hash_bytes
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>();
            let kind = if mode == "40000" {
                Kind::Tree
            } else {
                Kind::Blob
            };

            writeln!(handle, "{:0>6} {} {}    {}", mode, kind, hash_str, dir_name)
                .context("Failed to write dir_name to stdout")?;
        }
    }

    Ok(())
}
