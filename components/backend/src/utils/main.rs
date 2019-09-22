mod digital_ocean_api;

use std::env;
use dotenv::dotenv;
use uuid::Uuid;

use digital_ocean_api::{
    create_droplet,
    get_droplet_ip_address
};

#[derive(Debug)]
pub struct EnvironmentConfig {
    pub digital_ocean_token: String,
    pub digital_ocean_ssh_key_id: String
}

#[derive(Debug)]
pub struct WorkerMeta {
    pub id: String,
    pub name: String,
    pub ip_address: String
}

fn main() {
    let env_config = get_environment_config();

    println!("Creating worker...");
    let worker = create_worker(&env_config);
    println!("Success! {:#?}", worker);
}

fn get_environment_config() -> EnvironmentConfig {
    dotenv().ok();

    let mut config = EnvironmentConfig {
        digital_ocean_token: String::from(""),
        digital_ocean_ssh_key_id: String::from("")
    };

    for (key, value) in env::vars() {
        if key == "DIGITAL_OCEAN_TOKEN" {
            config.digital_ocean_token = value;
        } else if key == "DIGITAL_OCEAN_SSH_KEY_ID" {
            config.digital_ocean_ssh_key_id = value;
        }
    }

    config
}

fn create_worker(env_config: &EnvironmentConfig) -> WorkerMeta {
    let worker_id = Uuid::new_v4().to_hyphenated();
    let worker_name = format!("ruddy-worker-{}", worker_id);

    println!("Chose name: {}", worker_name);

    let droplet_id = create_droplet(&worker_name, env_config);

    println!("Sleeping...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let ip_address = get_droplet_ip_address(droplet_id, env_config);

    WorkerMeta {
        id: worker_id.to_string(),
        name: worker_name,
        ip_address: ip_address
    }
}
