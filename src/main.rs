use std::{env, error::Error, time::Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut args = env::args();
    let bucket = args.nth(1).ok_or("Specify a bucket name")?;
    let config = aws_config::load_from_env().await;
    let config_loaded = start.elapsed();
    println!("{:?}", config_loaded);
    let client = aws_sdk_s3::Client::new(&config);
    let _result = client.list_objects_v2().bucket(bucket).send().await?;
    let request_sent = start.elapsed();
    println!("{:?}", request_sent - config_loaded);
    Ok(())
}
