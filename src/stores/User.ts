import { writable } from "svelte/store";

type User = {
 
    id: string;
    name: string;
    email: string;
    role: string;
    specialization: string;
    patients: string[];
    patients_notes: string[];
    created_at: string;
    updated_at: string;
  
  };
  export const UserStore = writable<User[]>([]);