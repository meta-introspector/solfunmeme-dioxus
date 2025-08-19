use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::primitives::ByteStream;
use anyhow::Result;

pub async fn upload_object(client: &Client, bucket_name: &str, key: &str, body: Vec<u8>) -> Result<(), Error> {
    client.put_object()
        .bucket(bucket_name)
        .key(key)
        .body(ByteStream::from(body))
        .send()
        .await?;
    Ok(())
}

pub async fn download_object(client: &Client, bucket_name: &str, key: &str) -> Result<Vec<u8>, Error> {
    let resp = client.get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;
    let bytes = resp.body.collect().await?.into_bytes().to_vec();
    Ok(bytes)
}

pub async fn list_objects(client: &Client, bucket_name: &str) -> Result<Vec<String>, Error> {
    let resp = client.list_objects_v2()
        .bucket(bucket_name)
        .send()
        .await?;
    let mut keys = Vec::new();
    if let Some(contents) = resp.contents {
        for object in contents {
            if let Some(key) = object.key {
                keys.push(key);
            }
        }
    }
    Ok(keys)
}
