CREATE SCHEMA IF NOT EXISTS constructum;

CREATE TABLE constructum.repositories (
    id UUID PRIMARY KEY,
    git_id INTEGER UNIQUE NOT NULL,
    repo_url TEXT NOT NULL,
    repo_owner TEXT NOT NULL,
    repo_name TEXT NOT NULL,
    webhook_id INTEGER,
    enabled BOOLEAN NOT NULL,
    builds_executed INTEGER NOT NULL,
    CONSTRAINT valid_configuration CHECK (webhook_id IS NOT NULL OR enabled != TRUE)
);

CREATE TABLE constructum.jobs (
    id UUID PRIMARY KEY,
    seq INTEGER NOT NULL,
    repo_id UUID REFERENCES constructum.repositories NOT NULL,
    commit_id TEXT NOT NULL,
    is_finished BOOLEAN NOT NULL,
    status TEXT NOT NULL,
    UNIQUE (repo_id, seq)
);

CREATE TABLE constructum.steps (
    id UUID PRIMARY KEY,
    job UUID REFERENCES constructum.jobs NOT NULL,
    step_seq INTEGER NOT NULL,
    name TEXT NOT NULL,
    image TEXT NOT NULL,
    commands TEXT[] NOT NULL,
    status TEXT NOT NULL,
    log_keys TEXT[] NOT NULL,
    UNIQUE (job, step_seq)
);