use std::{error::Error, time::Instant};

use aws_config::BehaviorVersion;
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let profile = "hakusai";
    let bucket = "hakusai-test-bucket";

    let rustls_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http1()
        .build();

    let http_client = HyperClientBuilder::new().build(rustls_connector);

    let config = aws_config::ConfigLoader::default()
        .behavior_version(BehaviorVersion::latest())
        .profile_name(profile)
        .http_client(http_client)
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
