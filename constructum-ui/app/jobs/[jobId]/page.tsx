import Job from "../job";
import StepRow from "./step_row";

async function getJob(jobId: string) {
  const res = await fetch(
    `${process.env.CONSTRUCTUM_API_URL}/api/v1/jobs/${jobId}`
  );
  return res.json();
}

export default async function Page({
  params: { jobId },
}: {
  params: { jobId: string };
}) {
  const jobData: Job = await getJob(jobId);

  return (
    <>
      <h1>{jobData.job_uuid}</h1>
      <h2>Status: {jobData.status}</h2>
      <h2>Is Finished? {jobData.is_finished ? "Yes" : "No"}</h2>
      <div className="space-y-4">
        {jobData.steps.map((step, i) => (
          <StepRow step={step} />
        ))}
      </div>
    </>
  );
}
