import { ReportStore } from "../../../stores/Report";
import { invoke } from "@tauri-apps/api/core";

type ReportResponse = {
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

export async function getReports() {
  try {
    const data = await invoke<ReportResponse[]>("get_reports");
    console.log("Reports data:", data);

    const reports = data.map((report) => ({
      id: report.id,  
      body_type: report.body_type,
      condition: report.condition,
      created_at: report.created_at,
      patient: {
        id: report.patient.id,  
        name: report.patient.name
      },
      report_text: report.report_text,
      updated_at: report.updated_at
    }));
   
    reports.sort((a, b) => {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });

    ReportStore.set(reports);
  } catch (error) {
    console.error("Failed to load reports:", error);
  }
}