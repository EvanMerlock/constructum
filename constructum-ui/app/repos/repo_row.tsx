import Link from "next/link";
import Repo from "./repo";
import { RepoRegisterButton, RepoUnregisterButton } from "./repo_register_buttons";

export default function RepoRow({ repo }: { repo: Repo }) {
  return (
    <>
      <div className="p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
        {repo.id ? (
          <Link href={`/repos/${repo.id}`}>
            <h1>
              {repo.owner}/{repo.name}
            </h1>
          </Link>
        ) : (
          <h1>
            {repo.owner}/{repo.name}
          </h1>
        )}

        <ul>
          <li>Description: {repo.description}</li>
          <li>
            Link: <a href={repo.html_url}>{repo.html_url}</a>
          </li>
        </ul>
        {repo.is_registered ? <RepoUnregisterButton/> : <RepoRegisterButton/>}
      </div>
    </>
  );
}