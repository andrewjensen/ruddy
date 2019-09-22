use reqwest::{
    StatusCode,
    header
};
use serde::{
    Deserialize,
    Serialize
};

use crate::{
    EnvironmentConfig
};

const USER_DATA: &str = r###"
#cloud-config
runcmd:
  - echo "Hello from the cloud config" > /root/hello-world.txt
"###;

#[derive(Serialize)]
struct CreateDropletRequest {
    name: String,
    region: String,
    size: String,
    image: String,
    ssh_keys: Vec<String>,
    user_data: String
}

#[derive(Deserialize, Debug)]
struct CreateDropletResponse {
    droplet: DropletSummary
}

#[derive(Deserialize, Debug)]
struct DropletSummary {
    id: u64
}

pub fn create_droplet(name: &str, env_config: &EnvironmentConfig) -> u64 {
    println!("create_droplet()");

    let client = create_client(env_config);

    let request_body = CreateDropletRequest {
        name: String::from(name),
        region: String::from("nyc3"),
        size: String::from("c-4"),
        image: String::from("docker-18-04"),
        ssh_keys: vec![env_config.digital_ocean_ssh_key_id.clone()],
        user_data: String::from(USER_DATA)
    };
    let request_body_serialized = serde_json::to_string(&request_body)
        .expect("Could not serialize request body");

    let mut response = client.post("https://api.digitalocean.com/v2/droplets")
        .body(request_body_serialized)
        .send()
        .expect("Could not send request to create droplet");

    match response.status() {
        StatusCode::ACCEPTED => {
            println!("Success!");
        },
        status_code => {
            println!("Other code: {}", status_code);
        }
    }

    let response_body = response.text()
        .expect("Could not get response body from creating droplet");

    println!("Response body: {}", response_body);

    let response_body_json: CreateDropletResponse = serde_json::from_str(&response_body)
        .expect("Could not deserialize get droplet response");

    println!("Response body json: {:#?}", response_body_json);

    let droplet_id = response_body_json.droplet.id;
    println!("Droplet ID: {}", droplet_id);

    droplet_id
}

#[derive(Deserialize, Debug)]
struct GetDropletResponse {
    droplet: DropletDetails
}

#[derive(Deserialize, Debug)]
struct DropletDetails {
    id: u64,
    networks: DropletNetworks
}

#[derive(Deserialize, Debug)]
struct DropletNetworks {
    v4: Vec<DropletAddress>
}

#[derive(Deserialize, Debug)]
struct DropletAddress {
    ip_address: String
}

pub fn get_droplet_ip_address(droplet_id: u64, env_config: &EnvironmentConfig) -> String {
    println!("get_droplet_ip_address()");

    let client = create_client(env_config);

    let url = format!("https://api.digitalocean.com/v2/droplets/{}", droplet_id);
    let mut response = client.get(&url)
        .send()
        .expect("Could not send request to get droplet details");

    let response_body = response.text()
        .expect("Could not get response body from creating droplet");

    println!("Response body: {}", response_body);

    let response_body_json: GetDropletResponse = serde_json::from_str(&response_body)
        .expect("Could not deserialize get droplet response");

    println!("Response body json: {:#?}", response_body_json);

    let ip_address = &response_body_json.droplet.networks.v4[0].ip_address;

    ip_address.clone()
}

fn create_client(env_config: &EnvironmentConfig) -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", env_config.digital_ocean_token))
            .expect("Could not build auth header")
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json")
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Could not build Client");

    client
}
