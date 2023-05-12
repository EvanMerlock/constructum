import Job from "./job";

async function getJobs() {
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/jobs`)
    return res.json();
}

export default async function Page() {
    const jobData: Array<Job> = await getJobs();

    return (
        <>
        {jobData.map(function(job, i){
            return <JobRow job={job}/>;
        })}
        </>
    )
}

function JobRow({
    job
}: {
    job: Job,
}) {
    return <>
        <h1>{job.job_uuid}</h1>
        <ul>
            <li>Finished? {job.is_finished ? "Yes" : "No"}</li>
            <li>Status: {job.status}</li>
        </ul>
    </>
}