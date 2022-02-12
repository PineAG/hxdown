pub mod local;

use crate::err::HxError;
use async_trait::async_trait;
use std::marker::Send;

#[async_trait]
pub trait HxFS: Send {
    async fn mkdir(self: &Self, dir: &str) -> Result<(), HxError>;
    async fn is_dir(self: &Self, dir: &str) -> Result<bool, HxError>;
    async fn is_file(self: &Self, dir: &str, file: &str) -> Result<bool, HxError>;
    async fn write(self: &Self, dir: &str, file: &str, buf: bytes::Bytes) -> Result<(), HxError>;
}
