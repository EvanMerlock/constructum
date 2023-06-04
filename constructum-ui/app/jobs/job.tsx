import { UUID } from "crypto";

export default interface Job {
    job_uuid: UUID;
    repo_url: string;
    repo_name: string;
    commit_id: string;
    is_finished: boolean;
    status: JobStatus;
    steps: Array<JobStep>;
}
export enum JobStatus {
    InProgress,
    Complete,
    Failed
}

export interface JobStep {
    id: UUID;
    name: string;
    step_number: number;
    image: string;
    commands: Array<string>;
    status: StepStatus;
    log_key: Array<string> | undefined;
}

export enum StepStatus {
    NotStarted,
    InProgress,
    Success,
    Fail
}