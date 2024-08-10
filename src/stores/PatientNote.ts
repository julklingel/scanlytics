
import { writable } from "svelte/store";

type PatientNoteStore = {
  id: string;
  patient: string;
  symptoms: string;
  diagnosis: string;
  treatment: string;
  createdAt: string;
  updatedAt: string;
  severity: "Low" | "Medium" | "High";
  isUrgent: boolean;
  department: string;
  userOwner: string;
};

export const PatientNotesStore = writable<PatientNoteStore[]>([]);