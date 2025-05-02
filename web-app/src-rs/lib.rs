use co_noir::{Bn254, Rep3MpcNet, Utils};
use rand::{Rng, distributions::Alphanumeric};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use vercel_runtime::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Prover {
    user1: User,
    user2: User,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    age: u32,
    gender: u32,
    id: String,
    id_nullifier: u32,
    interests: Vec<u32>,
    region: u32,
    preferences: Preferences,
}

#[derive(Serialize, Deserialize, Debug)]
struct Preferences {
    age_max: u32,
    age_min: u32,
    gender: u32,
}

pub fn save_prover_data(prover: &Prover, as_user2: bool) -> Result<PathBuf, Error> {
    let mut toml = toml::to_string(prover)?;
    if as_user2 {
        toml = toml.replace("user1", "user2");
    }
    println!("toml: {}", toml);
    let file_path = write_file(None, &toml)?;
    Ok(file_path)
}

fn write_file(file_name: Option<&str>, data: &str) -> Result<PathBuf, Error> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tmp");

    // create the directory if it doesn't exist
    std::fs::create_dir_all(dir.clone())?;

    let name = match file_name {
        Some(file_name) => file_name.to_string(),
        None => rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect(),
    };

    let file = dir.join(name);
    std::fs::write(file.clone(), data)?;
    Ok(file)
}

pub fn split_input(circuit_path: &Path, prover_path: &Path) -> Result<Vec<Vec<u8>>, Error> {
    let program_artifact = Utils::get_program_artifact_from_file(circuit_path)?;
    let inputs = co_noir::parse_input(prover_path, &program_artifact)?;

    let mut rng = rand::thread_rng();
    let shares = co_noir::split_input_rep3::<Bn254, Rep3MpcNet, _>(inputs, &mut rng);

    let out = shares
        .iter()
        .map(|share| bincode::serialize(share))
        .collect::<Result<Vec<Vec<u8>>, _>>()
        .unwrap();

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_file() {
        let data = "test";

        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let file_name = format!("test_{}.txt", name);
        println!("file_name: {}", file_name);
        write_file(Some(&file_name), &data).unwrap();
    }
}
