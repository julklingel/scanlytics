import { writable } from "svelte/store";

type PatientNoteStore = {
  id: {
    [x: string]: any; String: string 
};
  patient: {
    id: { String: string };
    name: string;
  };
  symptoms: string;
  diagnosis: string;
  treatment: string;
  createdAt: string;
  updatedAt: string;
  severity: "low" | "medium" | "high";
  isUrgent: boolean;
  userOwner: {
    id: { String: string };
    name: string;
  };
};

export const PatientNotesStore = writable<PatientNoteStore[]>([]);