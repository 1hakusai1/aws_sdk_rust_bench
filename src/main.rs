use std::{error::Error, time::Instant};

use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let profile = "hakusai";
    let bucket = "hakusai-test-bucket";

    let config = aws_config::ConfigLoader::default()
        .behavior_version(BehaviorVersion::latest())
        .profile_name(profile)
        .load()
        .await;
    let config_loaded = start.elapsed();
    println!("{:?}", config_loaded);
    let client = aws_sdk_s3::Client::new(&config);
    let _result = client.list_objects_v2().bucket(bucket).send().await?;
    let request_sent = start.elapsed();
    println!("{:?}", request_sent - config_loaded);
    Ok(())
}
