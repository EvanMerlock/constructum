import Link from "next/link";
import Job from "./job";

export default function JobRow({ job }: { job: Job }) {
  return (
    <>
      <div className="flex flex-row p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
        <div className="basis-1/6">JOB</div>
        <div className="basis-4/6">
          <Link href={`/jobs/${job.job_uuid}`}>
            <h1>{job.job_uuid}</h1>
          </Link>
          <ul>
            <li>Finished? {job.is_finished ? "Yes" : "No"}</li>
            <li>Status: {job.status}</li>
          </ul>
        </div>
        <div className="basis-1/6">JOB ACTION</div>
      </div>
    </>
  );
}
