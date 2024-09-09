import { ReportStore } from "../../../stores/Report";
import { invoke } from "@tauri-apps/api/core";

type ReportResponse = {
  id: { String: string };
  report_text: string;
  body_part: string;
  condition: string;
  patient: {
    id: { String: string };
    name: string;
  };
  user_owner: {
    id: { String: string };
    name: string;
  };
  created_at: string;
  updated_at: string;
};

export async function getReports() {
  try {
    const data = await invoke<ReportResponse[]>("get_reports");
    console.log("Reports data:", data);

    const reports = data.map((report) => ({
      id: report.id,
      reportText: report.report_text,
      bodyPart: report.body_part,
      condition: report.condition,
      patient: {
        id: report.patient.id,
        name: report.patient.name
      },
      userOwner: {
        id: report.user_owner.id,
        name: report.user_owner.name
      },
      createdAt: report.created_at,
      updatedAt: report.updated_at
    }));
   
    reports.sort((a, b) => {
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });

    ReportStore.set(reports);
  } catch (error) {
    console.error("Failed to load reports:", error);
  }
}
