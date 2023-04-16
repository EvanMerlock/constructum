CREATE SCHEMA IF NOT EXISTS constructum;
CREATE TABLE constructum.jobs (
    id UUID PRIMARY KEY,
    repo_url TEXT NOT NULL,
    repo_name TEXT NOT NULL,
    commit_id TEXT NOT NULL,
    is_finished BOOLEAN NOT NULL,
    job_json JSON
);