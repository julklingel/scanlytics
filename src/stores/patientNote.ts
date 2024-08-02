import { writable } from "svelte/store";

type PatientNote = {
  id: string;
  patientName: string;
  patientId: string;
  symptoms: string;
  diagnosis: string;
  treatment: string;
  createdAt: string;
  updatedAt: string;
  severity: "Low" | "Medium" | "High";
  isUrgent: boolean;
  department: string;
  attendingDoctor: string;
};

export const patientNotes = writable<PatientNote[]>([]);