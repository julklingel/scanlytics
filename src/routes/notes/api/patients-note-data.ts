import { PatientNotesStore } from "../../../stores/PatientNote";
import { invoke } from "@tauri-apps/api/core";

type PatientNoteResponse = {
  id: { String: string };
  created_at: string;
  diagnosis: string;
  is_urgent: boolean;
  patient: {
    id: { String: string };
    name: string;
  };
  severity: "low" | "medium" | "high";
  symptoms: string;
  treatment: string;
  updated_at: string;
  user_owner: {
    id: { String: string };
    name: string;
  };
};

export async function getPatientNotes() {
  try {
    const data = await invoke<PatientNoteResponse[]>("get_patient_notes");
    console.log("Patient notes data:", data);

    const patientNotes = data.map((note) => ({
      id: note.id,
      patient: {
        id: note.patient.id,
        name: note.patient.name
      },
      symptoms: note.symptoms,
      diagnosis: note.diagnosis,
      treatment: note.treatment,
      createdAt: note.created_at,
      updatedAt: note.updated_at,
      severity: note.severity,
      isUrgent: note.is_urgent,
      userOwner: {
        id: note.user_owner.id,
        name: note.user_owner.name
      }
    }));
   
    patientNotes.sort((a, b) => {
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });

    PatientNotesStore.set(patientNotes);
  } catch (error) {
    console.error("Failed to load patient notes:", error);
  }
}