import Link from "next/link";
import Job from "./job";

export default function JobRow({ job }: { job: Job }) {
  return (
    <>
      <div className="p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
        <Link href={`/jobs/${job.job_uuid}`}><h1>{job.job_uuid}</h1></Link>
        <ul>
          <li>Finished? {job.is_finished ? "Yes" : "No"}</li>
          <li>Status: {job.status}</li>
        </ul>
      </div>
    </>
  );
}
