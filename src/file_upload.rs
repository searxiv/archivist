use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref UPLOAD_STORAGE: Arc<Mutex<HashMap<String, Vec<u8>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub async fn save_file(mut payload: Multipart, key: String) -> anyhow::Result<()> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut storage = UPLOAD_STORAGE.lock().await;
        storage.insert(key.clone(), Vec::new());

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            storage
                .get_mut(&key)
                .unwrap() // NOTE(mchernigin): key is inserted above
                .write_all(&data)
                .await?;
        }
    }

    Ok(())
}
