import { writable } from "svelte/store";

type User = {
 
  id: string;
  name: string;
  email: string;
  role: string;
  organization: string;
  patients: string[] | undefined;
  patient_notes: string[] | undefined;
  statements: string[] | undefined;
  images: string[] | undefined;
  reports: string[] | undefined;
  created_at: string;
  updated_at: string;
  
  };
  export const UserStore = writable<User[]>([]);