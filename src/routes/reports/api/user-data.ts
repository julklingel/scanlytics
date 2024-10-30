

import { UserStore } from "../../../stores/User";
import { invoke } from "@tauri-apps/api/core";




interface UserResponse {
  id: { id: string, tb: string };
  name: string;
  email: string;
  password: string;
  role: string;
  patients?: { id: string, tb: string }[];
  patient_notes?: { id: string, tb: string }[];
  images?: { id: string, tb: string }[];
  reports?: { id: string, tb: string }[];
  statements?: { id: string, tb: string }[];
  created_at: string;
  updated_at: string;
}





export async function getUsers() {
  try {
    const users: UserResponse[] = await invoke("get_users");
    const processedUsers = users.map((user) => ({
      id: user.id.id,
      name: user.name,
      email: user.email,
      role: user.role,
      patients: user.patients?.map(p => p.id),
      patient_notes: user.patient_notes?.map(n => n.id),
      statements: user.statements?.map(s => s.id),
      images: user.images?.map(i => i.id),
      reports: user.reports?.map(r => r.id),
      created_at: user.created_at,
      updated_at: user.updated_at,
    }));

    processedUsers.sort((a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    );
    UserStore.set(processedUsers);
  } catch (error) {
    console.error("Failed to load users:", error);
  }

}
