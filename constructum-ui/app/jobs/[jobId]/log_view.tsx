"use client";

import { UUID } from "crypto";
import useSWR from "swr";

const fetcher = (args: string) => fetch(args).then((res) => res.json());

interface LogViewData {
  ManyLogs: string[] | undefined;
  Logs: string | undefined;
}

export default function LogView({
  jobId,
  stepId,
}: {
  jobId: UUID;
  stepId: UUID;
}) {
  const {
    data,
    error,
    isLoading,
  }: { data: LogViewData; error: any; isLoading: boolean } = useSWR(
    `/v1/api/jobs/${jobId}/steps/${stepId}/logs`,
    fetcher
  );

  if (error) {
    return (
      <>
        <div className="space-y-4">
          <h1>Error Occurred While Loading</h1>
        </div>
      </>
    );
  }

  if (isLoading) {
    return (
      <>
        <div className="space-y-4">
          <h1>Loading</h1>
        </div>
      </>
    );
  }

  const logs = (
    data.ManyLogs != undefined ? data.ManyLogs.join() : data.Logs
  )?.split("\n");

  if (logs == undefined) {
    return (
      <>
        <div className="space-y-4">
          <h1>Error Occurred While Loading</h1>
        </div>
      </>
    );
  }

  const numDigits = logs.length.toString().length;

  return (
    <>
      <div className="px-2 bg-slate-700 flex flex-row logView">
        <pre className="basis-1/12">
          {logs.map((_line, i) => {
            const spacing = numDigits - (i + 1).toString().length;
            return (
              <>
                <code className="text-slate-50">
                  {i + 1} {" ".repeat(spacing)}|<br />
                </code>
              </>
            );
          })}
        </pre>
        <pre className="basis-11/12">
          <>
            {logs.map((line, _i) => {
              return (
                <>
                  <code className="text-slate-50">{line}</code>
                  <br />
                </>
              );
            })}
          </>
        </pre>
      </div>
    </>
  );
}
