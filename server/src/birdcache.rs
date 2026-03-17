use object_store::{ObjectStoreExt, PutPayload, local::LocalFileSystem, path::Path};

#[derive(Debug)]
pub enum BirdCacheError {
    CacheMiss,
    CacheWriteFail,
}

pub async fn get_bird_from_cache(bird_seed: &str) -> Result<String, BirdCacheError> {
    let store: LocalFileSystem = LocalFileSystem::new_with_prefix("bird_gltfs").unwrap();
    let path = Path::from(format!("{}.gltf", bird_seed));
    let cache_store_result = store.get(&path).await;
    if let Ok(cache_store_value) = cache_store_result {
        let bytes = cache_store_value
            .bytes()
            .await
            .expect("failed to read bytes");
        Ok(String::from_utf8_lossy(&bytes).to_string())
    } else {
        Err(BirdCacheError::CacheMiss)
    }
}

pub async fn add_bird_to_cache(gltf_str: &str, bird_seed: &str) -> Result<(), BirdCacheError> {
    let store: LocalFileSystem = LocalFileSystem::new_with_prefix("bird_gltfs").unwrap();
    let path = Path::from(format!("{}.gltf", bird_seed));

    let payload = PutPayload::from_bytes(gltf_str.to_string().into_bytes().into());
    let write_result = store.put(&path, payload).await;
    if write_result.is_ok() {
        Ok(())
    } else {
        Err(BirdCacheError::CacheWriteFail)
    }
}
