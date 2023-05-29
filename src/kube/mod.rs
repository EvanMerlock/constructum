use std::{path::PathBuf, pin::Pin};

use futures::{Stream, FutureExt};
use k8s_openapi::api::{batch::v1::Job, core::v1::{PersistentVolumeClaim, Pod}};
use kube::{Api, api::{LogParams, ListParams, DeleteParams}};
use s3::Bucket;
use serde::{Serialize, Deserialize};
use tokio::{fs::File, io::AsyncReadExt};
use uuid::Uuid;
use tokio_stream::StreamExt;
use bytes::Bytes;

pub mod utils;
mod secret;

pub use self::secret::*;

use crate::{pipeline::{PipelineJobConfig}, client::PipelineExecError};

pub fn build_client_pvc(pipeline_uuid: Uuid) -> Result<PersistentVolumeClaim, serde_json::Error> {
    serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "PersistentVolumeClaim",
        "metadata": {
            "name": format!("pipeline-{pipeline_uuid}-pvc"),
            "namespace": "constructum",
            "annotations": {
                "volume.beta.kubernetes.io/storage-class": "nfs-ephemeral-client"
            },
            "labels": {
                "constructum-pipeline": format!("{pipeline_uuid}")
            },
        },
        "spec": {
            "accessModes": [ "ReadWriteMany" ],
            "resources": {
                "requests": {
                    "storage": "2Gi"
                }
            },
            "storageClassName": "nfs-ephemeral-client"
        }
    }))
}

pub fn build_client_job(pipeline_uuid: Uuid, pipeline_client_name: String, container_name: String, service_account_name: Option<String>) -> Result<Job, serde_json::Error> {
    serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": pipeline_client_name,
            "namespace": "constructum",
        },
        "spec": {
            "backoffLimit": 0,
            "template": {
                "metadata": {
                    "name": format!("{pipeline_client_name}-pod"),
                },
                "spec": {
                    "serviceAccountName": service_account_name,
                    "containers": [{
                        "name": format!("{pipeline_client_name}-container"),
                        "image": container_name,
                        "envFrom": [ 
                            {
                                "configMapRef": {
                                    "name": "constructum-cfg"
                                }
                            }
                        ],
                        "env": [
                            {
                                "name": "CONSTRUCTUM_PIPELINE_UUID",
                                "value": pipeline_uuid.to_string(),
                            }
                        ],
                        "volumeMounts": [
                            {
                                "mountPath": "/data",
                                "name": "data-pvc"
                            },
                            {
                                "mountPath": "/var/run/secrets/tokens",
                                "name": "vault-token"
                            }
                        ]
                    }],
                    "volumes": [
                        {
                            "name": "data-pvc",
                            "persistentVolumeClaim": {
                                "claimName": format!("pipeline-{pipeline_uuid}-pvc")
                            }
                        },
                        {
                            "name": "vault-token",
                            "projected": {
                                "sources": [{
                                    "serviceAccountToken": {
                                        "path": "vault-token",
                                        "expirationSeconds": 7200,
                                        "audience": "vault",
                                    }
                                }]
                            }
                        }
                    ],
                    "restartPolicy": "Never",
                }
            }
        }
    }))
}

pub fn build_pipeline_job(job_cfg: PipelineJobConfig) -> Result<(Job, String, String), serde_json::Error> {
    // TODO: this might exceed the k8s resource limit. re-encode the uuid.
    let pipeline_job_name = format!("pipeline-{}-{}", job_cfg.pipeline, job_cfg.step);
    let sa_name = match job_cfg.annotations.is_some() {
        true => Some("constructum-client-build"),
        false => None,
    };
    let secret_env_from = match job_cfg.annotations.is_some() {
        true => Some(job_cfg.annotations.unwrap().to_serde_values()),
        false => None,
    };

    let container_name = format!("{}-container", job_cfg.step);

    Ok((serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": pipeline_job_name,
            "namespace": "constructum",
        },
        "spec": {
            "backoffLimit": 0,
            "template": {
                "metadata": {
                    "name": format!("{pipeline_job_name}-pod"),
                    "annotations": secret_env_from,
                },
                "spec": {
                    "serviceAccountName": sa_name,
                    "containers": [{
                        "name": container_name,
                        "image": job_cfg.container,
                        "volumeMounts": [{
                            "mountPath": "/data",
                            "name": "data-pvc"
                        }],
                        "command": [ "/bin/sh" ],
                        "args": job_cfg.commands,
                        "workingDir": format!("{}", job_cfg.pipeline_working_directory.display())
                    }],
                    "volumes": [{
                        "name": "data-pvc",
                        "persistentVolumeClaim": {
                            "claimName": format!("pipeline-{}-pvc", job_cfg.pipeline)
                        }
                    }],
                    "restartPolicy": "Never",
                }
            }
        }
    }))?, pipeline_job_name, container_name))
}

pub async fn put_pod_logs_to_s3(job_name: String, container_name: Option<String>, file_name: String, s3_bucket: Bucket) -> Result<Vec<String>, kube::Error> {
    let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
    let pods: Api<Pod> = Api::namespaced(k8s_client, "constructum");
    let params = ListParams::default().labels(&format!("job-name={job_name}"));
    let mut log_names = Vec::new();
    for pod in pods.list(&params).await? {
        let pod_name = pod.metadata.name.expect("failed to get pod name");
        // this may fail due to k8s errors?
        let mut lp = LogParams::default();
        lp.container = container_name.clone();
        let log_string = pods.logs(&pod_name, &lp).await?;
        let log_file_name = format!("{pod_name}-{file_name}.txt");
        log_names.push(log_file_name.clone());
        s3_bucket.put_object(log_file_name, log_string.as_bytes()).await.expect("failed to write container logs to s3");
    }
    Ok(log_names)
}

pub struct PodLog {
    pub job_name: String,
    pub container_name: Option<String>,
    pub log: Bytes,
}

pub async fn stream_pod_logs(job_name: String, container_name: Option<String>) -> Result<impl Stream<Item = Result<PodLog, kube::Error>>, kube::Error> {
    let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
    let pods: Api<Pod> = Api::namespaced(k8s_client, "constructum");
    let params = ListParams::default().labels(&format!("job-name={job_name}"));
    let mut stream: Pin<Box<dyn Stream<Item = Result<PodLog, kube::Error>>>> = Box::pin(tokio_stream::empty());
    for pod in pods.list(&params).await? {
        let pod_name = pod.metadata.name.expect("failed to get pod name");
        // this may fail due to k8s errors?
        let lp = LogParams { container: container_name.clone(), ..Default::default() };
        let cnt_name = container_name.clone();
        let jname = job_name.clone();
        let log_stream = Box::pin(pods.log_stream(&pod_name, &lp).await?.map(move |x: Result<Bytes, kube::Error>| 
            match x {
                Ok(log) => Ok(PodLog {
                    job_name: jname.clone(),
                    container_name: cnt_name.clone(),
                    log,
                }),
                Err(e) => Err(e)
        }));
        stream = Box::pin(stream.merge(log_stream));
    }
    Ok(stream)
}

pub async fn delete_job(job_name: &str) -> Result<(), kube::Error> {
    let k8s_client = kube::Client::try_default().await?;

    let jobs: Api<Job> = Api::namespaced(k8s_client.clone(), "constructum");
    let pods: Api<Pod> = Api::namespaced(k8s_client, "constructum");

    let params = ListParams::default().labels(&format!("job-name={job_name}"));
    for pod in pods.list(&params).await? {
        pods.delete(&pod.metadata.name.expect("failed to pull pod name"), &DeleteParams::background()).await?;
    }

    jobs.delete(job_name, &DeleteParams::background()).await?;

    Ok(())
}

pub async fn delete_pvc(pipeline_uuid: &str) -> Result<(), kube::Error> {
    let k8s_client = kube::Client::try_default().await?;

    let pvcs: Api<PersistentVolumeClaim> = Api::namespaced(k8s_client, "constructum");

    let params = ListParams::default().labels(&format!("constructum-pipeline={pipeline_uuid}"));
    for pvc in pvcs.list(&params).await? {
        pvcs.delete(&pvc.metadata.name.expect("failed to pull pvc name"), &DeleteParams::background()).await?;
    }

    Ok(())
}

pub async fn read_kubernetes_token(vault_url: String, location: PathBuf) -> Result<String, PipelineExecError> {
    let mut fs = File::open(location).await?;

    let mut var = String::new();

    fs.read_to_string(&mut var).await?;


    // TODO: Replace with post util
    let http_client = reqwest::ClientBuilder::new();

    #[derive(Debug, Serialize, Deserialize)]
    struct VaultTokenPayload {
        jwt: String,
        role: String
    }

    let http_client = http_client.build()?;
    let req = http_client
        .post(format!("{vault_url}/v1/auth/kubernetes/login"))
        .json(&VaultTokenPayload {
            jwt: var,
            role: String::from("constructum-validate"),
        })
        .build()?;
    let resp = http_client.execute(req).await?;

    #[derive(Debug, Serialize, Deserialize)]

    struct VaultTokenAuthResponse {
        client_token: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct VaultTokenResponse {
        auth: VaultTokenAuthResponse
    }

    let tok = resp.json::<VaultTokenResponse>().await?;

    Ok(tok.auth.client_token)
}