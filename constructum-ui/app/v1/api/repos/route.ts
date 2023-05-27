import { authOptions } from "@/pages/api/auth/[...nextauth]";
import { getServerSession } from "next-auth";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const session = await getServerSession(authOptions);

  if (session === null) {
    return null;
  }

  const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/repos`, {
    headers: {
      // @ts-ignore
      Authorization: "token " + session.access_token,
    },
  });
  return NextResponse.json(await res.json());
}

export async function POST(request: Request) {
    const req = await request.json();
    const session = await getServerSession(authOptions);
  
    if (session === null) {
      return null;
    }
  
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/repos`, {
      headers: {
        // @ts-ignore
        Authorization: "token " + session.access_token,
        "Content-Type": "application/json; charset=utf-8"
      },
      method: "POST",
      body: JSON.stringify({
        "owner": req.owner,
        "name": req.name,
      }),
    });

    if (res.status > 299 || res.status < 200) {
      return NextResponse.json(await res.json(), { status: res.status })
    }

    return NextResponse.json(await res.json())
}