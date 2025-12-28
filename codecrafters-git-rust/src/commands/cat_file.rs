use std::io::{self, Write};

use anyhow::Context;

use crate::objects::Object;

pub(crate) fn invoke(
    pretty_print: bool,
    file_type: bool,
    size: bool,
    object_hash: &str,
) -> anyhow::Result<()> {
    let mut object = Object::read(object_hash)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if pretty_print {
        io::copy(&mut object.reader, &mut handle)
            .context("pretty_print: Failed to write bytes to stdout")?;
    }

    if file_type {
        write!(handle, "{}", object.kind).context("file_type: Failed to write kind to stdout")?;
    }

    if size {
        write!(handle, "{}", object.expected_size)
            .context("size: Failed to write kind to stdout")?;
    }

    Ok(())
}
