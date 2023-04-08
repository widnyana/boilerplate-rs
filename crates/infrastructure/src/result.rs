use core::result::Result as CoreResult;

use crate::errors::ChetError;

pub type Result<T> = CoreResult<T, ChetError>;
