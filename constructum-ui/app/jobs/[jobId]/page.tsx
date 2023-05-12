import { UUID } from "crypto";
import Job from "../job";

async function getJob(jobId: string) {
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/job/${jobId}`)
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
        </>
    )
}