import { writable } from "svelte/store";

type ReportStore = {
  id: { String: string }; 
  body_type: string;
  condition: string;
  created_at: string;
  patient: {
    id: { String: string };
    name: string;
  };
  report_text: string;
  updated_at: string;
};

export const ReportStore = writable<ReportStore[]>([]);