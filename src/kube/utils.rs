use k8s_openapi::api::batch::v1::Job;
use kube::runtime::wait::Condition;

pub fn is_job_failed() -> impl Condition<Job> {
    |obj: Option<&Job>| {
        if let Some(job) = &obj {
            if let Some(s) = &job.status {
                if let Some(conds) = &s.conditions {
                    if let Some(pcond) = conds.iter().find(|c| c.type_ == "Failed") {
                        return pcond.status == "True";
                    }
                }
            }
        }
        false
    }
}