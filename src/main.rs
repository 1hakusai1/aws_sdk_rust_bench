use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let bucket = args.nth(1).ok_or("Specify a bucket name")?;
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let result = client.list_objects_v2().bucket(bucket).send().await?;
    println!("{:?}", result);
    Ok(())
}
