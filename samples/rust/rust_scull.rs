// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

use kernel::file;
use kernel::prelude::*;

struct Scull;

#[vtable]
impl file::Operations for Scull {
    fn open(_context: &Self::OpenData, _file: &file::File) -> Result<Self::Data> {
        pr_info!("File was opened\n");
        Ok(())
    }
}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world\n");
        Ok(Scull)
    }
}

module! {
    type: Scull,
    name: "scull",
    license: "GPL v2",
}
