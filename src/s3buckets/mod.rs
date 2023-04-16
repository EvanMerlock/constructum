use s3::Bucket;

pub async fn get_file_from_s3(file_name: String, s3_bucket: Bucket) -> Result<String, s3::error::S3Error> {
    let obj = s3_bucket.get_object(file_name).await?;
    // TODO: inefficient; extra copying
    let result = String::from_utf8(obj.bytes().iter().copied().collect()).expect("failed to convert to utf8");
    Ok(result)
}