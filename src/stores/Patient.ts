import { writable } from "svelte/store";

type Patient = {
  id: string;
  name: string;
  date_of_birth: string;
  gender: string;
  contact_number: string;
  address: string;
  primary_doctor: {
    id: string;
    name: string;
  };
  created_at: string;
  updated_at: string;
};

export const PatientStore = writable<Patient[]>([]);