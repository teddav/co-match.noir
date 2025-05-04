use axum::extract::Multipart;
use co_noir::Rep3AcvmType;
use rand::{Rng, distributions::Alphanumeric};
use std::collections::BTreeMap;

use crate::matching::DIR;

const MAX_SHARE_SIZE: usize = 1024;

pub type Share = BTreeMap<String, Rep3AcvmType<ark_bn254::Fr>>;

pub async fn upload(mut multipart: Multipart) -> Result<String, Box<dyn std::error::Error>> {
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

    let dir = DIR.join("tmp");

    // TODO: remove this
    std::fs::remove_dir_all(&dir)?;

    std::fs::create_dir_all(&dir)?;
    let user_id = random_id();

    for (i, share) in shares.into_iter().enumerate() {
        let file_name = format!("{}-{}", user_id, i);
        let file_path = dir.join(file_name);
        std::fs::write(file_path, share)?;
    }

    Ok(user_id)
}

pub fn get_shares(id: &str) -> Result<[Share; 3], Box<dyn std::error::Error + Send + Sync>> {
    let share0 = bin_to_share(std::fs::read(DIR.join(format!("{id}-0")))?)?;
    let share1 = bin_to_share(std::fs::read(DIR.join(format!("{id}-1")))?)?;
    let share2 = bin_to_share(std::fs::read(DIR.join(format!("{id}-2")))?)?;
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
