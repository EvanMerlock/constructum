use std::pin::Pin;

use futures::{Stream};
use redis::AsyncCommands;
use tokio_stream::StreamExt;
use bytes::Bytes;
use tracing::error;

use crate::{kube::{PodLog}, client::ConstructumClientError};

async fn logs_to_redis(redis_client: redis::Client, mut recv: tokio::sync::mpsc::Receiver<impl Stream<Item = Result<PodLog, kube::Error>>>) -> Result<(), ConstructumClientError> {
    let mut connection = redis_client.get_async_connection().await.expect("failed to connect to redis");
    let mut stream: Pin<Box<dyn Stream<Item = Result<PodLog, kube::Error>>>> = Box::pin(tokio_stream::empty());

    // run while channel is still open AND any stream is still open
    loop {
        tokio::select! {
            Some(stream_2) = recv.recv() => {
                stream = Box::pin(stream.merge(stream_2));
            },
            Some(val) = stream.next() => {
                match val {
                    Ok(log) => {
                        let log_key = format!("job:{}:step:{}", log.job_name, log.container_name.map_or(String::from(""), |x| x));
                        let log_base = grab_existing_log_from_redis(&mut connection, log_key.clone()).await.expect("failed to get log base from redis");
                        let new_log = build_new_log(log_base, log.log).await;
                        put_log_to_redis(&mut connection, log_key.clone(), new_log).await.expect("failed to put log into redis");
                    },
                    Err(e) => {
                        error!("Error encountered during redis log stream: {}", e)
                    }
                }
            }
            else => { break; }
        }
    }

    Ok(())
}

async fn grab_existing_log_from_redis(connection: &mut redis::aio::Connection, log_key: String) -> Result<String, redis::RedisError> {
    if connection.exists(&log_key).await? {
        Ok(connection.get(&log_key).await?)
    } else {
        Ok(String::new())
    }
}

async fn build_new_log(mut log_base: String, new_line: Bytes) -> String {
    let new_str = String::from_utf8(new_line.to_vec()).expect("failed to convert new line");
    log_base.push_str(&new_str);
    log_base
}

async fn put_log_to_redis(connection: &mut redis::aio::Connection, log_key: String, log: String) -> Result<(), redis::RedisError> {
    connection.set(log_key, log).await?;
    Ok(())
}