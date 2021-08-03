use super::{HxFS};
use super::super::err::HxError;
use tokio::io;
use tokio::fs::{File, create_dir_all, metadata};
use std::path::Path;
use async_trait::async_trait;


pub struct LocalFS<'a> {
    pub root: &'a Path
}

impl LocalFS<'_> {
    pub fn new(root: &'_ str) -> LocalFS<'_> {
        return LocalFS {root: Path::new(root)}
    }
}

#[async_trait]
impl HxFS for LocalFS<'_> {
    async fn mkdir(&self, path: &str) -> Result<(), HxError> {
        let res = create_dir_all(self.root.join(path)).await;
        return match res {
            Ok(()) => Result::Ok(()),
            Err(err) => Result::Err(HxError::warp(true, Box::new(err)))
        }
    }
    async fn is_dir(self: &Self, path: &str) -> Result<bool, HxError> {
        let fp = self.root.join(path);
        let exists = fp.exists();
        if !exists { return Ok(false) };
        let meta = metadata(fp).await.expect("Unknown Error");
        Ok(meta.is_dir())
    }
    async fn is_file(self: &Self, path: &str) -> Result<bool, HxError> {
        let fp = self.root.join(path);
        let exists = fp.exists();
        if !exists { return Ok(false) };
        let meta = metadata(fp).await.expect("Unknown Error");
        Ok(meta.is_file())
    }
    async fn write(self: &Self, path: &str, buf: bytes::Bytes) -> Result<(), HxError> {
        let fp = self.root.join(path);
        let mut f = File::create(fp).await.expect("Failed to create file.");
        let mut src = std::io::Cursor::new(buf);
        io::copy(&mut src, &mut f).await.expect("Failed to write file.");
        Ok(())
    }
}

