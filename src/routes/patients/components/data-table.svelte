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
  import { patients } from "../../../stores/patient";

  type Patient = {
    id: string;
    name: string;
    patient_id: string;
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
  const infoDescription: string | null | never = "There are no patients available. Please add a new patient.";
  let dataAvailable: boolean;

  $: dataAvailable;

  const filterValue = writable("");

  onMount(async () => {
    try {
      const data = await invoke<Patient[]>("get_patients");
      if (data.length === 0) {
        dataAvailable = false;
      } else {
        dataAvailable = true;
      }
      patients.set(data);
      filteredPatients = data;
    } catch (error) {
      console.error("Failed to load patients:", error);
    }
  });

  $: filteredPatients = $filterValue
    ? $patients.filter((patient) =>
        patient.name.toLowerCase().includes($filterValue.toLowerCase())
      )
    : $patients;

  function handleCreateNewPatient() {
    goto("./patients/new");
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }

  function handlePatientView(id: string) {
    goto(`./patients/${id}`);
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
          <Table.TableHead>Patient ID</Table.TableHead>
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
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{patient.name}</Table.TableCell>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{patient.patient_id}</Table.TableCell>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{formatDate(patient.date_of_birth)}</Table.TableCell>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{patient.gender}</Table.TableCell>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{patient.contact_number}</Table.TableCell>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{patient.primary_doctor.name}</Table.TableCell>
            <Table.TableCell on:click={() => handlePatientView(patient.id)}>{formatDate(patient.created_at)}</Table.TableCell>
            <Table.TableCell>
              <DataTableActions id={patient.id} patientId={patient.patient_id} />
            </Table.TableCell>
          </Table.TableRow>
        {/each}
      </Table.TableBody>
    </Table.Table>
  </div>
{/if}