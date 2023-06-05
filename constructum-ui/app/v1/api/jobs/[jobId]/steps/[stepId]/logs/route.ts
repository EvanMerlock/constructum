import { authOptions } from "@/pages/api/auth/[...nextauth]";
import { UUID } from "crypto";
import { getServerSession } from "next-auth";
import { NextResponse } from "next/server";

export async function GET(request: Request, {
    params
}: { params: { jobId: UUID, stepId: UUID } }) {
    const session = await getServerSession(authOptions);
  
    if (session === null) {
      return null;
    }
  
    const res = await fetch(`${process.env.CONSTRUCTUM_API_URL}/api/v1/jobs/${params.jobId}/steps/${params.stepId}/logs`, {
        cache: 'no-store',
    //   headers: {
    //     // @ts-ignore
    //     Authorization: "token " + session.access_token,
    //   },
    });
    return NextResponse.json(await res.json());
  }