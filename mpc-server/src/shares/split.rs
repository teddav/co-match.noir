use co_noir::{Bn254, Rep3MpcNet};
use noirc_artifacts::program::ProgramArtifact;
use rand::{Rng, distributions::Alphanumeric};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProverData {
    user1: User,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    age: u32,
    gender: u32,
    id: String,
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

pub async fn split_handler(
    payload: ProverData,
    program_artifact: &ProgramArtifact,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let prover_path1 = save_prover_data(&payload, false)?;
    let prover_path2 = save_prover_data(&payload, true)?;

    let shares1 = split_input(prover_path1, &program_artifact)?;
    let shares2 = split_input(prover_path2, &program_artifact)?;

    let mut out = shares1
        .iter()
        .map(|d| hex::encode(d))
        .collect::<Vec<String>>();

    let out2 = shares2
        .iter()
        .map(|d| hex::encode(d))
        .collect::<Vec<String>>();

    out.extend(out2);

    Ok(out)
}

fn split_input(
    input_path: PathBuf,
    program_artifact: &ProgramArtifact,
) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let inputs = co_noir::parse_input(input_path, &program_artifact)?;

    let mut rng = rand::thread_rng();
    let shares = co_noir::split_input_rep3::<Bn254, Rep3MpcNet, _>(inputs, &mut rng);

    let out = shares
        .iter()
        .map(|share| bincode::serialize(share))
        .collect::<Result<Vec<Vec<u8>>, _>>()
        .unwrap();

    Ok(out)
}

fn save_prover_data(
    prover: &ProverData,
    as_user2: bool,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut toml = toml::to_string(prover)?;
    if as_user2 {
        toml = toml.replace("user1", "user2");
    }
    println!("toml: {}", toml);
    let file_path = write_file(None, &toml)?;
    Ok(file_path)
}

fn write_file(
    file_name: Option<&str>,
    data: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync + 'static>> {
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
