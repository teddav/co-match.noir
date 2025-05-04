use axum::extract::Multipart;
use co_noir::Rep3AcvmType;
use rand::{Rng, distributions::Alphanumeric};
use std::collections::BTreeMap;

use crate::{
    db::{connect_db, does_hash_exist, insert_user},
    matching::{SHARES_DIR_1, SHARES_DIR_2},
    token::encode_token,
};

const MAX_SHARE_SIZE: usize = 1024;

pub type Share = BTreeMap<String, Rep3AcvmType<ark_bn254::Fr>>;

pub async fn upload(
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

    if shares.len() != 6 {
        return Err("Invalid number of shares".into());
    }

    let dir1 = SHARES_DIR_1.clone();
    let dir2 = SHARES_DIR_2.clone();

    // TODO: remove this
    std::fs::remove_dir_all(&dir1)?;
    std::fs::remove_dir_all(&dir2)?;

    std::fs::create_dir_all(&dir1)?;
    std::fs::create_dir_all(&dir2)?;

    let shares1 = shares[..3].to_vec();
    let shares2 = shares[3..].to_vec();

    let user_id = random_id();

    let conn = connect_db()?;

    let hash = format!("{:x}", md5::compute(shares2.concat()));
    if does_hash_exist(&conn, &hash)? {
        return Err("Hash already exists".into());
    }

    for (i, share) in shares1.iter().enumerate() {
        let file_name = format!("{}-{}", user_id, i);
        let file_path = dir1.join(file_name);
        std::fs::write(file_path, share)?;
    }
    for (i, share) in shares2.iter().enumerate() {
        let file_name = format!("{}-{}", user_id, i);
        let file_path = dir2.join(file_name);
        std::fs::write(file_path, share)?;
    }

    insert_user(&conn, &user_id, &twitter_handle, &hash)?;

    let token = encode_token(user_id)?;

    Ok(token)
}

pub fn get_shares(
    id: &str,
    user1: bool,
) -> Result<[Share; 3], Box<dyn std::error::Error + Send + Sync>> {
    let dir = if user1 {
        SHARES_DIR_1.clone()
    } else {
        SHARES_DIR_2.clone()
    };
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
