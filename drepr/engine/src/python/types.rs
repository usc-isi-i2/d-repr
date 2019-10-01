use readers::prelude::RAReader;
use std::ptr::NonNull;

pub struct ReaderPtr(pub NonNull<dyn RAReader>);
unsafe impl std::marker::Send for ReaderPtr {}
