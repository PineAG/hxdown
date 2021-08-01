pub mod local;

use crate::err::HxError;
use async_trait::async_trait;

#[async_trait]
pub trait HxFS {
    async fn mkdir(self: &Self, path: &str) -> Result<(), HxError>;
    async fn is_dir(self: &Self, path: &str) -> Result<bool, HxError>;
    async fn is_file(self: &Self, path: &str) -> Result<bool, HxError>;
    async fn write(self: &Self, path: &str, buf: bytes::Bytes) -> Result<(), HxError>;
}
