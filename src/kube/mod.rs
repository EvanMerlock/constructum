use k8s_openapi::api::{batch::v1::Job, core::v1::{PersistentVolumeClaim, Pod}};
use kube::{Api, api::{LogParams, ListParams, DeleteParams}};
use s3::Bucket;
use uuid::Uuid;

pub mod utils;

use crate::pipeline::PipelineJobConfig;

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

pub fn build_client_job(pipeline_uuid: Uuid, pipeline_client_name: String, container_name: String) -> Result<Job, serde_json::Error> {
    serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": pipeline_client_name,
        },
        "spec": {
            "backoffLimit": 0,
            "template": {
                "metadata": {
                    "name": format!("{pipeline_client_name}-pod"),
                },
                "spec": {
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
                        "volumeMounts": [{
                            "mountPath": "/data",
                            "name": "data-pvc"
                        }]
                    }],
                    "volumes": [{
                        "name": "data-pvc",
                        "persistentVolumeClaim": {
                            "claimName": format!("pipeline-{pipeline_uuid}-pvc")
                        }
                    }],
                    "restartPolicy": "Never",
                    "serviceAccount": "constructum-client-build",
                }
            }
        }
    }))
}

pub fn build_pipeline_job(job_cfg: PipelineJobConfig) -> Result<(Job, String), serde_json::Error> {
    let pipeline_job_name = format!("pipeline-{}-{}", job_cfg.pipeline, job_cfg.step);
    Ok((serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": pipeline_job_name,
        },
        "spec": {
            "backoffLimit": 0,
            "template": {
                "metadata": {
                    "name": format!("{pipeline_job_name}-pod")
                },
                "spec": {
                    "containers": [{
                        "name": format!("{pipeline_job_name}-container"),
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
    }))?, pipeline_job_name))
}


// TODO: ref pods via job controller UID
pub async fn put_pod_logs_to_s3(job_name: String, file_name: String, s3_bucket: Bucket) -> Result<(), kube::Error> {
    let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
    let pods: Api<Pod> = Api::namespaced(k8s_client, "constructum");
    let params = ListParams::default().labels(&format!("job-name={job_name}"));
    for pod in pods.list(&params).await? {
        let pod_name = pod.metadata.name.expect("failed to get pod name");
        let log_string = pods.logs(&pod_name, &LogParams::default()).await.expect("failed to get job logs");
        let log_file_name = format!("{pod_name}-{file_name}.txt");
        s3_bucket.put_object(log_file_name, log_string.as_bytes()).await.expect("failed to write container logs to s3");
    }
    Ok(())
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