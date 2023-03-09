// Copyright (c) 2021 slog-try developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `Buffer` drain used in testing

use slog::{Drain, Never, OwnedKVList, Record};
use std::{
    fmt,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub(crate) struct Buffer {
    buffer: Arc<RwLock<Vec<u8>>>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            buffer: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let read = self.buffer.read().unwrap();
        let output = String::from_utf8_lossy(&read);
        write!(f, "{}", output)
    }
}

impl Drain for Buffer {
    type Ok = ();
    type Err = Never;

    fn log(&self, record: &Record<'_>, _logger_values: &OwnedKVList) -> Result<Self::Ok, Never> {
        let mut guard = match self.buffer.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.extend(format!("{}", record.msg()).as_bytes().iter());

        Ok(())
    }
}
