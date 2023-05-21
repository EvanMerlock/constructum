import Job from "./job";
import JobRow from "./job_row";

async function getJobs() {
  const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/jobs`);
  return res.json();
}

export default async function Page() {
  const jobData: Array<Job> = await getJobs();

  return (
    <>
      <div className="space-y-4">
        {jobData.map(function (job, i) {
          return <JobRow job={job} />;
        })}
      </div>
    </>
  );
}
