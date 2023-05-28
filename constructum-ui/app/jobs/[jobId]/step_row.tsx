import { JobStep } from "../job";

export default function StepRow({ step }: { step: JobStep }) {
    return (
      <>
        <div className="p-6 w-full mx-auto bg-slate-100 rounded-xl shadow-lg">
          <ul>
            <li>Name: {step.name}</li>
            <li>Status: {step.status}</li>
          </ul>
        </div>
      </>
    );
  }
  