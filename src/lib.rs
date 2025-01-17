mod env;
mod string_ptr;
mod tools;

pub use env::Env;
pub use string_ptr::StringPtr;
pub use tools::abort;

use std::fmt;
use wasmer::Memory;

pub trait Read<T> {
    fn read(self, memory: &Memory) -> anyhow::Result<T>;
    fn size(self, memory: &Memory) -> anyhow::Result<u32>;
}

pub trait Write<T> {
    fn alloc(value: &str, memory: &Env) -> anyhow::Result<Box<Self>>;
    fn write(&mut self, value: &str, env: &Env) -> anyhow::Result<Box<Self>>;
    fn free(self, memory: &Env) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub enum Error {
    Mem(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Mem(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
