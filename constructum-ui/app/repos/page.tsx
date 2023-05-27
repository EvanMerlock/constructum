"use client";

import useSWR from "swr";
import Repo from "./repo";
import RepoRow from "./repo_row";

const fetcher = (args: string) => fetch(args).then((res) => res.json());

async function getRepos() {
  const res = await fetch(`/v1/api/repos`);
  return res.json();
}

export default function RepoPage() {
  const { data, error, isLoading } = useSWR("/v1/api/repos", fetcher);

  if (error) {
    return (<>
      <div className="space-y-4">
        <h1>Error Occurred While Loading</h1>
      </div>
    </>);
  }

  if (isLoading) {
    return (<>
      <div className="space-y-4">
        <h1>Loading</h1>
      </div>
    </>);
  }

  // TODO: Add pagination
  return (
    <>
      <div className="space-y-4">
        {data.map(function (repo: Repo, _i: any) {
          return <RepoRow repo={repo} />;
        })}
      </div>
    </>
  );
}
