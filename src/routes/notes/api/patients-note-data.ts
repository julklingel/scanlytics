import { PatientNotesStore } from "../../../stores/PatientNote";
import { invoke } from "@tauri-apps/api/core";

type PatientNote = {
  id: string;
  patient: string;
  symptoms: string;
  diagnosis: string;
  treatment: string;
  created_at: string;
  updated_at: string;
  severity: "Low" | "Medium" | "High";
  is_urgent: boolean;
  department: string;
  user_owner: string;
};

export async function getPatientNotes() {
  try {
    const data = await invoke<PatientNote[]>("get_patient_notes");
    const patientNotes = data.map((note) => ({
      id: note.id,
      patient: note.patient,
      symptoms: note.symptoms,
      diagnosis: note.diagnosis,
      treatment: note.treatment,
      createdAt: note.created_at,
      updatedAt: note.updated_at,
      severity: note.severity,
      isUrgent: note.is_urgent,
      department: note.department,
      userOwner: note.user_owner
    }));
    
    patientNotes.sort((a, b) => {
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });

    PatientNotesStore.set(patientNotes);
  } catch (error) {
    console.error("Failed to load patient notes:", error);
  }
}