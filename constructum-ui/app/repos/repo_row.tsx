import Link from "next/link";
import Repo from "./repo";
import {
  RepoRegisterButton,
  RepoUnregisterButton,
} from "./repo_register_buttons";

export default function RepoRow({ repo }: { repo: Repo }) {
  return (
    <>
      <div className="flex flex-row p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
        <div className="basis-1/6">
          REPO
        </div>
        <div className="basis-4/6">
          {(repo.id && repo.is_registered) ? (
            <Link href={`/repos/${repo.id}`}>
              <h1>
                {repo.owner.login}/{repo.name}
              </h1>
            </Link>
          ) : (
            <h1>
              {repo.owner.login}/{repo.name}
            </h1>
          )}

          <ul>
            <li>Description: {repo.description}</li>
            <li>
              Link: <a href={repo.html_url}>{repo.html_url}</a>
            </li>
          </ul>
        </div>
        <div className="basis-1/6">
          {repo.is_registered ? (
            <RepoUnregisterButton repo={repo} />
          ) : (
            <RepoRegisterButton repo={repo} />
          )}
        </div>
      </div>
    </>
  );
}
