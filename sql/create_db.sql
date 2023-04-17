CREATE SCHEMA IF NOT EXISTS constructum;

CREATE TABLE constructum.jobs (
    id UUID PRIMARY KEY,
    repo_url TEXT NOT NULL,
    repo_name TEXT NOT NULL,
    commit_id TEXT NOT NULL,
    is_finished BOOLEAN NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE constructum.steps (
    id UUID PRIMARY KEY,
    job UUID REFERENCES constructum.jobs NOT NULL,
    name TEXT NOT NULL,
    image TEXT NOT NULL,
    commands TEXT[] NOT NULL,
    status TEXT NOT NULL,
    log_keys TEXT[] NOT NULL
);