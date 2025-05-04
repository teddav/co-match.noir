use axum::extract::Multipart;
use co_noir::Rep3AcvmType;
use rand::{Rng, distributions::Alphanumeric};
use std::{collections::BTreeMap, sync::Arc};

use crate::{
    AppState,
    db::{does_hash_exist, insert_user},
    matching::{DIR, SHARES_DIR},
    token::encode_token,
};

const MAX_SHARE_SIZE: usize = 1024;

pub type Share = BTreeMap<String, Rep3AcvmType<ark_bn254::Fr>>;

pub async fn upload(
    state: Arc<AppState>,
    twitter_handle: String,
    mut multipart: Multipart,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut shares = Vec::new();
    while let Some(field) = multipart.next_field().await? {
        let data = field.bytes().await?;
        println!("data: {data:?}");

        if data.len() > MAX_SHARE_SIZE {
            return Err("Invalid share size".into());
        }

        shares.push(data);
    }

    if shares.len() != 3 {
        return Err("Invalid number of shares".into());
    }

    let dir = SHARES_DIR.clone();

    // TODO: remove this
    std::fs::remove_dir_all(&dir)?;

    std::fs::create_dir_all(&dir)?;
    let user_id = random_id();

    let conn = state.conn.lock().unwrap();

    let hash = format!("{:x}", md5::compute(shares.concat()));
    if does_hash_exist(&conn, &hash)? {
        return Err("Hash already exists".into());
    }

    for (i, share) in shares.iter().enumerate() {
        let file_name = format!("{}-{}", user_id, i);
        let file_path = dir.join(file_name);
        std::fs::write(file_path, share)?;
    }

    insert_user(&conn, &user_id, &twitter_handle, &hash)?;

    let token = encode_token(user_id)?;

    Ok(token)
}

pub fn get_shares(id: &str) -> Result<[Share; 3], Box<dyn std::error::Error + Send + Sync>> {
    let dir = SHARES_DIR.clone();
    let share0 = bin_to_share(std::fs::read(dir.join(format!("{id}-0")))?)?;
    let share1 = bin_to_share(std::fs::read(dir.join(format!("{id}-1")))?)?;
    let share2 = bin_to_share(std::fs::read(dir.join(format!("{id}-2")))?)?;
    Ok([share0, share1, share2])
}

fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}

fn bin_to_share(bin: Vec<u8>) -> Result<Share, Box<dyn std::error::Error + Send + Sync>> {
    let share: Share = bincode::deserialize(&bin)?;
    Ok(share)
}
