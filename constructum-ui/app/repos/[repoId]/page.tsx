import { UUID } from "crypto";
import Repo from "../repo";

async function getRepo(repoId: string) {
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/repos/${repoId}`)
    return res.json();
} 

interface ConstructumRepo {
    repo_uuid: UUID,
    git_id: number,
    repo_url: string,
    repo_owner: string,
    repo_name: string,
    webhook_id: number,
    enabled: boolean
}

export default async function Page({
    params: { repoId },
}: {
    params: { repoId: string };
}) {
    const repoData: ConstructumRepo = await getRepo(repoId);

    return (
        <>
            <h1>{repoData.repo_owner}/{repoData.repo_name}</h1>
            <h2>ID: {repoData.repo_uuid}</h2>
            <h2>URL: {repoData.repo_url}</h2>
        </>
    )
}