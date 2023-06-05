"use client";

import { useState } from "react";

export default function Details({
  summaryProps,
  detailsProps,
}: {
  summaryProps: React.ReactNode;
  detailsProps: React.ReactNode;
}) {
  const [active, setIsActive] = useState(false);

  return (
    <>
      <div className="p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg space-y-2">
        <div className="flex flex-row" onClick={() => setIsActive(!active)}>
          <div className="basis-1/12">STEP STATUS</div>
          <div className="basis-10/12">{summaryProps}</div>
          <div className="basis-1/12 flex flex-row place-content-center">
            {active ? "▲" : "▼"}
          </div>
        </div>
        {active && (
          <>
            <hr />
            {detailsProps}
          </>
        )}
      </div>
    </>
  );
}
