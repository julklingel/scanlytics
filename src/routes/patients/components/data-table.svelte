<script lang="ts">
  import DataTableActions from "./data-table-actions.svelte";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import PlusIcon from "lucide-svelte/icons/plus";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import InfoMsg from "../../components/ui/infomodal.svelte";
  import { PatientStore } from "../../../stores/Patient";
  import { UserStore } from "../../../stores/User";

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

  let filteredPatients: Patient[] = [];
  const infoTitle: string | null | never = "No patients found";
  const infoDescription: string | null | never =
    "There are no patients available. Please add a new patient.";
  let dataAvailable: boolean;

  $: dataAvailable;

  const filterValue = writable("");

  interface UserResponse {
  id: { id: string, tb: string };
  name: string;
  email: string;
  password: string;
  role: string;
  organization: { id: string, tb: string };
  patients?: { id: string, tb: string }[];
  patient_notes?: { id: string, tb: string }[];
  images?: { id: string, tb: string }[];
  reports?: { id: string, tb: string }[];
  created_at: string;
  updated_at: string;
}

  onMount(async () => {
    try {
    const users: UserResponse[] = await invoke("get_users");
    const processedUsers = users.map((user) => ({
      id: user.id.id,
      name: user.name,
      email: user.email,
      role: user.role,
      organization: user.organization.id,
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

    console.log(processedUsers);
    UserStore.set(processedUsers);
  } catch (error) {
    console.error("Failed to load users:", error);
  }

    try {
      const data = await invoke<Patient[]>("get_patients");

      if (data.length === 0) {
        dataAvailable = false;
      } else {
        dataAvailable = true;
      }

      const patients = data.map((patient) => ({
        id: patient.id.id.String,
        name: patient.name,
        date_of_birth: patient.date_of_birth,
        gender: patient.gender,
        contact_number: patient.contact_number,
        address: patient.address,
        primary_doctor: {
          id: patient.primary_doctor?.id || "",
          name: patient.primary_doctor?.name || "No doctor assigned",
        },
        created_at: patient.created_at,
        updated_at: patient.updated_at,
      }));

      patients.sort((a, b) => {
        return (
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        );
      });

      PatientStore.set(patients);
      filteredPatients = patients;
    } catch (error) {
      console.error("Failed to load patients:", error);
    }
  });

  $: filteredPatients = $filterValue
    ? $PatientStore.filter((patient) =>
        patient.name.toLowerCase().includes($filterValue.toLowerCase()),
      )
    : $PatientStore;

  function handleCreateNewPatient() {
    goto("./patients/new");
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }

  function handlePatientView(id: string) {
    goto(`./patients/${id}`);
  }

  function getDoctorName(doctorId: string) {
    let doctorName = "No doctor assigned";
    UserStore.subscribe(users => {
      const doctor = users.find((d: any) => d.id === doctorId);
      if (doctor) {
        doctorName = doctor.name;
      }
    })();
    return doctorName;
  }

  function getPatientName(patientId: string) {
    let patientName = "No patient assigned";
    PatientStore.subscribe(patients => {
      const patient = patients.find((p: Patient) => p.id === patientId);
      if (patient) {
        patientName = patient.name;
      }
    })();
    return patientName;
  }
</script>

{#if !dataAvailable}
  <div class="flex flex-col gap-4">
    <InfoMsg {infoTitle} {infoDescription} />

    <Button on:click={handleCreateNewPatient}>
      <PlusIcon size={16} />
      <span>Add New Patient</span>
    </Button>
  </div>
{:else}
  <div>
    <div class="flex gap-4 mb-2">
      <Button on:click={handleCreateNewPatient}>
        <PlusIcon size={16} />
        <span>Add New Patient</span>
      </Button>
      <Input
        class="max-w-sm"
        placeholder="Filter patient names..."
        type="text"
        bind:value={$filterValue}
      />
    </div>
    <Table.Table>
      <Table.TableHeader>
        <Table.TableRow>
          <Table.TableHead>Name</Table.TableHead>
   
          <Table.TableHead>Date of Birth</Table.TableHead>
          <Table.TableHead>Gender</Table.TableHead>
          <Table.TableHead>Contact Number</Table.TableHead>
          <Table.TableHead>Primary Doctor</Table.TableHead>
          <Table.TableHead>Created At</Table.TableHead>
          <Table.TableHead>Actions</Table.TableHead>
        </Table.TableRow>
      </Table.TableHeader>
      <Table.TableBody>
        {#each filteredPatients as patient}
          <Table.TableRow>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}
              >{patient.name}</Table.TableCell
            >
    
            <Table.TableCell on:click={() => handlePatientView(patient.id)}
              >{formatDate(patient.date_of_birth)}</Table.TableCell
            >
            <Table.TableCell on:click={() => handlePatientView(patient.id)}
              >{patient.gender}</Table.TableCell
            >
            <Table.TableCell on:click={() => handlePatientView(patient.id)}
              >{patient.contact_number}</Table.TableCell
            >
            <Table.TableCell on:click={() => handlePatientView(patient.id)}
              >{getDoctorName(patient.primary_doctor.id.String)}</Table.TableCell
            >
            <Table.TableCell on:click={() => handlePatientView(patient.id)}
              >{formatDate(patient.created_at)}</Table.TableCell
            >
            <Table.TableCell>
              <DataTableActions id={patient.id} />
            </Table.TableCell>
          </Table.TableRow>
        {/each}
      </Table.TableBody>
    </Table.Table>
  </div>
{/if}
