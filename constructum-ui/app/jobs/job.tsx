import { UUID } from "crypto";

export default class Job {
    private _job_uuid: UUID;
    private _repo_url: string;
    private _repo_name: string;
    private _commit_id: string;
    private _is_finished: boolean;
    private _status: JobStatus;
    private _steps: Array<JobStep>;


    public get job_uuid(): UUID {
        return this._job_uuid;
    }
    public set job_uuid(value: UUID) {
        this._job_uuid = value;
    }

    public get repo_url(): string {
        return this._repo_url;
    }
    public set repo_url(value: string) {
        this._repo_url = value;
    }

    public get repo_name(): string {
        return this._repo_name;
    }
    public set repo_name(value: string) {
        this._repo_name = value;
    }

    public get commit_id(): string {
        return this._commit_id;
    }
    public set commit_id(value: string) {
        this._commit_id = value;
    }

    public get is_finished(): boolean {
        return this._is_finished;
    }
    public set is_finished(value: boolean) {
        this._is_finished = value;
    }

    public get status(): JobStatus {
        return this._status;
    }
    public set status(value: JobStatus) {
        this._status = value;
    }

    public get steps(): Array<JobStep> {
        return this._steps;
    }
    public set steps(value: Array<JobStep>) {
        this._steps = value;
    }


    constructor(job_uuid: UUID, repo_url: string, repo_name: string, commit_id: string, is_finished: boolean, status: JobStatus, steps: Array<JobStep>) {
        this._job_uuid = job_uuid;
        this._repo_name = repo_name;
        this._repo_url = repo_url;
        this._commit_id = commit_id;
        this._is_finished = is_finished;
        this._status = status;
        this._steps = steps;
    }
}

export enum JobStatus {
    InProgress,
    Complete,
    Failed
}

export class JobStep {
    private _name: string;
    private _step_number: number;
    private _image: string;
    private _commands: Array<string>;
    private _status: StepStatus;
    private _log_key: Array<string> | undefined;


    public get name(): string {
        return this._name;
    }
    public set name(value: string) {
        this._name = value;
    }

    public get step_number(): number {
        return this._step_number;
    }
    public set step_number(value: number) {
        this._step_number = value;
    }

    public get image(): string {
        return this._image;
    }
    public set image(value: string) {
        this._image = value;
    }

    public get commands(): Array<string> {
        return this._commands;
    }
    public set commands(value: Array<string>) {
        this._commands = value;
    }

    public get status(): StepStatus {
        return this._status;
    }
    public set status(value: StepStatus) {
        this._status = value;
    }

    public get log_key(): Array<string> | undefined {
        return this._log_key;
    }
    public set log_key(value: Array<string> | undefined) {
        this._log_key = value;
    }

    constructor(name: string, step_number: number, image: string, commands: Array<string>, status: StepStatus, log_key: Array<string> | undefined) {
        this._name = name;
        this._step_number = step_number;
        this._commands = commands;
        this._status = status;
        this._image = image;
        this._log_key = log_key;
    }
}

export enum StepStatus {
    NotStarted,
    InProgress,
    Success,
    Fail
}