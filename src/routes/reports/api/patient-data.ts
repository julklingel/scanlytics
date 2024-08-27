
import { PatientStore } from "../../../stores/Patient";
import { invoke } from "@tauri-apps/api/core";


type Patient = {
  id: string;
  name: string;
  date_of_birth: string;
  gender: string;
  contact_number: string;
  address: string;
  created_at: string;
  updated_at: string;
};




export async function getPatients() {
  try {
    const data = await invoke<Patient[]>("get_patients");
    const patients = data.map((patient) => ({
      id: patient.id.id.String,
      name: patient.name,
      date_of_birth: patient.date_of_birth,
      gender: patient.gender,
      contact_number: patient.contact_number,
      address: patient.address,
      created_at: patient.created_at,
      updated_at: patient.updated_at,
    }));

    patients.sort((a, b) => {
      return (
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      );
    });
  
    PatientStore.set(patients);

  } catch (error) {
    console.error("Failed to load patients:", error);
  }

}