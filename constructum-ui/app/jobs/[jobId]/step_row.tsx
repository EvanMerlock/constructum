import { JobStep, StepStatus } from "../job";
import { UUID } from "crypto";
import LogView from "./log_view";
import Details from "./details";

// TODO: only lazy load LogView
export default function StepRow({
  jobId,
  step,
}: {
  jobId: UUID;
  step: JobStep;
}) {
  return (
    <>
      {step.status == StepStatus.NotStarted ? (
        <div className="p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
          <ul>
            <li>ID: {step.id}</li>
            <li>Name: {step.name}</li>
            <li>Status: {step.status}</li>
          </ul>
        </div>
      ) : (
        <Details
          summaryProps={
            <ul>
              <li>ID: {step.id}</li>
              <li>Name: {step.name}</li>
              <li>Status: {step.status}</li>
            </ul>
          }
          detailsProps={<LogView jobId={jobId} stepId={step.id} />}
        />
      )}
    </>
  );
}
