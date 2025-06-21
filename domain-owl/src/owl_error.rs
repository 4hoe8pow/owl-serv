use std::error::Error;

pub trait OwlError: Error + Send + Sync {}

impl<T> OwlError for T where T: Error + Send + Sync {}
