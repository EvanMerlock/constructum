"use client";

import { UUID } from "crypto";
import Repo from "./repo";
import { useSWRConfig } from "swr";
import useSWRMutation from 'swr/mutation'

async function registerRepository(url: string, { arg }: { arg: Repo }) {  
    const res = await fetch(url, {
      method: "POST",
      body: JSON.stringify({
        "owner": arg.owner.login,
        "name": arg.name,
      }),
    });

    return res.json();
  }

  async function unregisterRepository(url: string, { arg }: { arg: UUID }) {  
    const res = await fetch(`${url}/${arg}`, {
      method: "DELETE",
    });
    return res.json();
  }


export function RepoRegisterButton({repo} : {repo: Repo}) {
  const { trigger } = useSWRMutation('/v1/api/repos', registerRepository, {})

    return (
        <button onClick={() => trigger(repo)} className="w-full h-full mx-auto bg-blue-400 rounded-xl shadow-lg">REGISTER</button>
    )
}

export function RepoUnregisterButton({repo} : {repo: Repo}) {
    const { trigger } = useSWRMutation('/v1/api/repos', unregisterRepository, {})

    return (
        <button onClick={() => trigger(repo.id)} className="w-full h-full mx-auto bg-red-400 rounded-xl shadow-lg">UNREGISTER</button>
    )
}