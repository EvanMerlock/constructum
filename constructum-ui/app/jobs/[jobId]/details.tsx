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
      <div className="p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
        <div onClick={() => setIsActive(!active)}>
          {active ? "▲" : "▼"} <br />
          {summaryProps}
        </div>
        {active && detailsProps}
      </div>
    </>
  );
}
