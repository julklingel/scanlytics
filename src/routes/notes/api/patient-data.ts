import { PatientStore } from "../../../stores/Patient";
import { invoke } from "@tauri-apps/api/core";

interface PatientResponse {
  id: { id: string, tb: string };
  name: string;
  date_of_birth: string;
  gender: string;
  contact_number: string;
  address: string;
  notes?: { id: string, tb: string }[];
  reports?: { id: string, tb: string }[];
  images?: { id: string, tb: string }[];
  created_at: string;
  updated_at: string;
}

export async function getPatients() {
  try {
    const patients: PatientResponse[] = await invoke("get_patients");
    const processedPatients = patients.map((patient) => ({
      id: patient.id.id,
      name: patient.name,
      date_of_birth: patient.date_of_birth,
      gender: patient.gender,
      contact_number: patient.contact_number,
      address: patient.address,
      notes: patient.notes?.map(n => n.id),
      reports: patient.reports?.map(r => r.id),
      images: patient.images?.map(i => i.id),
      created_at: patient.created_at,
      updated_at: patient.updated_at,
    }));
    processedPatients.sort((a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    );
    console.log(processedPatients);
    PatientStore.set(processedPatients);
  } catch (error) {
    console.error("Failed to load patients:", error);
  }
}