use std::{pin::Pin, time::Duration};

use futures::{Stream};
use redis::AsyncCommands;
use tokio_stream::StreamExt;
use bytes::Bytes;
use tracing::{error, info};

use crate::{kube::{PodLog, stream_pod_logs}};

use self::error::ConstructumRedisError;

pub mod error;

pub async fn logs_to_redis(redis_client: redis::Client, job_name: String, container_name: String, phase_name: String) -> Result<(), ConstructumRedisError> {
    tokio::time::sleep(Duration::from_millis(5000)).await;
    let mut stream = stream_pod_logs(job_name, Some(container_name)).await?;
    let mut connection = redis_client.get_async_connection().await.expect("failed to connect to redis");


    info!("starting redis");
    // run while channel is still open / stream is still open
    // TODO: should not publish logs every time a new log comes in. check duration across stream awaits; buffer and only publish at a refresh interval
    while let Some(val) = stream.next().await {
        info!("got log");
        match val {
            Ok(log) => {
                let log_key = format!("job:{}:step:{}", log.job_name, phase_name.clone());
                let log_base = grab_existing_log_from_redis(&mut connection, log_key.clone()).await.expect("failed to get log base from redis");
                let new_log = build_new_log(log_base, log.log).await;
                put_log_to_redis(&mut connection, log_key.clone(), new_log).await.expect("failed to put log into redis");
            },
            Err(e) => {
                error!("Error encountered during redis log stream: {}", e)
            }
        }
    }

    info!("done");

    Ok(())
}

pub async fn grab_log_from_redis(redis_client: redis::Client, job_name: String, phase_name: String) -> Result<Option<String>, ConstructumRedisError> {
    let mut connection = redis_client.get_async_connection().await.expect("failed to connect to redis");
    let log_key = format!("job:{job_name}:step:{phase_name}");
    if connection.exists(&log_key).await? {
        Ok(Some(connection.get(&log_key).await?))
    } else {
        Ok(None)
    }
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
    // sets exp to 30m by default
    // should prevent issues/overfilling Redis
    connection.set_ex(log_key, log, 1800).await?;
    Ok(())
}

// pub async fn delete_log_from_redis(redis_client: redis::Client, job_name: String, phase_name: String) -> Result<(), ConstructumRedisError> {
//     let log_key = format!("job:pipeline-{job_name}:step:{phase_name}");
//     let mut connection = redis_client.get_async_connection().await.expect("failed to connect to redis");
//     delete_existing_log_from_redis(&mut connection, log_key).await?;

//     Ok(())
// }

// async fn delete_existing_log_from_redis(connection: &mut redis::aio::Connection, log_key: String) -> Result<(), redis::RedisError> {
//     if connection.exists(&log_key).await? {
//         connection.get_del(log_key).await?
//     }

//     Ok(())
// }