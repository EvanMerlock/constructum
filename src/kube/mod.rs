use k8s_openapi::api::{batch::v1::Job, core::v1::{PersistentVolumeClaim, Pod}};
use kube::{Api, api::LogParams};
use s3::Bucket;
use uuid::Uuid;

use crate::pipeline::PipelineJobConfig;

pub fn build_client_pvc(pipeline_uuid: Uuid) -> Result<PersistentVolumeClaim, serde_json::Error> {
    serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "PersistentVolumeClaim",
        "metadata": {
            "name": format!("pipeline_{pipeline_uuid}_pvc"),
            "namespace": "constructum",
            "annotations": {
                "volume.beta.kubernetes.io/storage-class": "nfs-ephemeral-client"
            }
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

pub fn build_client_job(pipeline_uuid: Uuid, pipeline_client_name: String, container_name: String,) -> Result<Job, serde_json::Error> {
    serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": pipeline_client_name,
        },
        "spec": {
            "template": {
                "metadata": {
                    "name": format!("{pipeline_client_name}-pod")
                },
                "spec": {
                    "containers": [{
                        "name": format!("{pipeline_client_name}-container"),
                        "image": container_name,
                        "envFrom": {
                            "configMapRef": {
                                "name": "constructum-cfg"
                            }
                        },
                        "volumeMounts": [{
                            "mountPath": "/data",
                            "name": "data-pvc"
                        }]
                    }],
                    "volumes": [{
                        "name": "data-pvc",
                        "persistentVolumeClaim": {
                            "claimName": format!("pipeline_{pipeline_uuid}_pvc")
                        }
                    }],
                    "restartPolicy": "Never",
                }
            }
        }
    }))
}

pub fn build_pipeline_job(job_cfg: PipelineJobConfig) -> Result<Job, serde_json::Error> {
    let pipeline_job_name = format!("{}-{}", job_cfg.pipeline, job_cfg.step);
    serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": pipeline_job_name,
        },
        "spec": {
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
                        "workingDir": "/data"
                    }],
                    "volumes": [{
                        "name": "data-pvc",
                        "persistentVolumeClaim": {
                            "claimName": format!("pipeline_{}_pvc", job_cfg.pipeline)
                        }
                    }],
                    "restartPolicy": "Never",
                }
            }
        }
    }))
}

pub async fn put_pod_logs_to_s3(pod_name: String, file_name: String, s3_bucket: Bucket) -> Result<(), kube::Error> {
    let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
    let pods: Api<Pod> = Api::namespaced(k8s_client, "constructum");
    let log_string = pods.logs(&pod_name, &LogParams::default()).await.expect("failed to get job logs");
    let log_file_name = format!("{file_name}.txt");
    s3_bucket.put_object(log_file_name, log_string.as_bytes()).await.expect("failed to write container logs to s3");
    Ok(())
}