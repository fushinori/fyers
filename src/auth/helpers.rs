use sha2::{Digest, Sha256};
use url::Url;

pub fn compute_app_id_hash(app_id: &str, app_secret: &str) -> String {
    let input = format!("{app_id}:{app_secret}");

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    let hash = hasher.finalize();
    hex::encode(hash)
}

pub fn get_query_param(url: &Url, key: &str) -> Option<String> {
    url.query_pairs()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.into_owned())
}
