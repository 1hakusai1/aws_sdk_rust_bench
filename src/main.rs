use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let bucket = args.nth(1).ok_or("Specify a bucket name")?;
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let result = client.list_objects().bucket(bucket).send().await?;
    let file_names = result
        .contents()
        .iter()
        .filter_map(|o| o.key())
        .collect::<Vec<_>>();
    dbg!(file_names);
    Ok(())
}
