import { getServerSession } from "next-auth";
import Repo from "./repo";
import RepoRow from "./repo_row";
import { authOptions } from "@/pages/api/auth/[...nextauth]";

async function getRepos() {
  const session = await getServerSession(authOptions);
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/repos`, {
      headers: {
        "Authorization": "token BLAH"
      }
    });
    const text = await res.text()
    console.log(text)
    return res.json();
}

export default async function RepoPage() {
    const repoData: Array<Repo> = await getRepos();

    return (
        <>
          <div className="space-y-4">
            {repoData.map(function (repo, i) {
              return <RepoRow repo={repo} />;
            })}
          </div>
        </>
      );
}