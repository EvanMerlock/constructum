import { authOptions } from "@/pages/api/auth/[...nextauth]";
import { UUID } from "crypto";
import { getServerSession } from "next-auth";
import { NextResponse } from "next/server";

export async function DELETE(request: Request, {
    params,
}: {
    params: { repoId: UUID }
}) {
    const session = await getServerSession(authOptions);
  
    if (session === null) {
        return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });    
    }
  
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/v1/repos/${params.repoId}`, {
      headers: {
        // @ts-ignore
        Authorization: "token " + session.access_token,
      },
      method: "DELETE",
    });

    if (res.status != 204) {
        return NextResponse.json({ error: 'Internal Server Error' }, { status: res.status });    
    }

    return NextResponse.json({})
}