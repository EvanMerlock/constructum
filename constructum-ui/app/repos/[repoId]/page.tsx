import Job from "@/app/jobs/job";
import JobRow from "@/app/jobs/job_row";
import { UUID } from "crypto";

async function getRepo(repoId: string) {
  const res = await fetch(
    `${process.env.CONSTRUCTUM_API_URL}/api/v1/repos/${repoId}`
  );
  return res.json();
}

async function getJobsForRepo(repoId: string) {
  const res = await fetch(
    `${process.env.CONSTRUCTUM_API_URL}/api/v1/repos/${repoId}/jobs`,
    { cache: 'no-store' }
  );
  return res.json();
}

interface ConstructumRepo {
  repo_uuid: UUID;
  git_id: number;
  repo_url: string;
  repo_owner: string;
  repo_name: string;
  webhook_id: number;
  enabled: boolean;
}

export default async function Page({
  params: { repoId },
}: {
  params: { repoId: string };
}) {
  const repoData: ConstructumRepo = await getRepo(repoId);
  const jobData: Array<Job> = await getJobsForRepo(repoId);

  return (
    <>
      <h1>
        {repoData.repo_owner}/{repoData.repo_name}
      </h1>
      <h2>ID: {repoData.repo_uuid}</h2>
      <h2>URL: {repoData.repo_url}</h2>
      <h1>Jobs run on this repository:</h1>
      <div className="space-y-4">
        {jobData.map(function (job, i) {
          return <JobRow job={job} />;
        })}
      </div>
    </>
  );
}
