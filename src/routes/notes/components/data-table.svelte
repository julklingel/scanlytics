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
  import { patientNotes } from "../../../stores/PatientNote";
  import { PatientStore } from "../../../stores/Patient";
  import { UserStore } from "../../../stores/User";
  

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


  type Options = {
    year: "numeric" | "2-digit";
    month: "numeric" | "2-digit" | "long" | "short" | "narrow";
    day: "numeric" | "2-digit";
    hour: "numeric" | "2-digit";
    minute: "numeric" | "2-digit";
  };

  let filteredNotes: PatientNote[] = [];
  const infoTitle: string | null | never  = "Uupps, no patient notes found";
  const infoDescription: string | null | never  =
    "There are no patient notes available. Please create a new note.";
  let dataAvailable:boolean;

  $: dataAvailable
  

  const filterValue = writable("");

  onMount(async () => {
    try {
      const data = await invoke<any[]>("get_patient_notes");
      if (data.length === 0) {
        dataAvailable = false;
      } else {
        dataAvailable = true;
      }

      const notes = data.map((note) => ({
        id: note.id.id.String,
        patientName: note.patient.id.String,
        patientId: note.patient.id.String,
        symptoms: note.symptoms,
        diagnosis: note.diagnosis,
        treatment: note.treatment,
        createdAt: note.created_at,
        updatedAt: note.updated_at,
        severity: note.severity,
        isUrgent: note.is_urgent,
        department: note.department,
        attendingDoctor: note.attending_doctor.id.String,
      }));
      notes.sort((a, b) => {
        return (
          new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
        );
      });
      patientNotes.set(notes);
      filteredNotes = notes;
    } catch (error) {
      console.error("Failed to load patient notes:", error);
    }
    try {
      const userData: any = await invoke("get_users");
      const users = userData.map((user: any) => ({
        id: user.id.id.String,
        name: user.name,
        email: user.email,
        role: user.role,
        created_at: user.created_at,
        updated_at: user.updated_at,
      }));

      users.sort((a: any, b: any) => {
        return (
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        );
      });

      UserStore.set(users);
    } catch (error) {
      console.error("Failed to load users:", error);
    }

    try {
      const data = await invoke<Patient[]>("get_patients");

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
     
    } catch (error) {
      console.error("Failed to load patients:", error);
    }
  });


  $: filteredNotes = $filterValue
    ? $patientNotes.filter((note) =>
        note.patientName.toLowerCase().includes($filterValue.toLowerCase()),
      )
    : $patientNotes;


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




  function handleCreateNewNote() {
    goto("./notes/new");
  }

  function formatDate(dateString: string) {
    const options: Options = {
      year: "numeric",
      month: "long",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    };
    return new Date(dateString).toLocaleDateString(undefined, options);
  }

  function handleNoteView(id: string) {
    goto(`./notes/${id}`);
  }

</script>

{#if !dataAvailable}
  <div class="flex flex-col gap-4">
    <InfoMsg {infoTitle} {infoDescription} />

    <Button on:click={handleCreateNewNote}>
      <PlusIcon size={16} />
      <span>Create New Note</span>
    </Button>
  </div>
{:else}
  <div>
    <div class="flex gap-4 mb-2">
      <Button on:click={handleCreateNewNote}>
        <PlusIcon size={16} />
        <span>Create New Note</span>
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
          <Table.TableHead>Created at</Table.TableHead>
          <Table.TableHead>Patient Name</Table.TableHead>
       
          <Table.TableHead>Department</Table.TableHead>
          <Table.TableHead>Attending Doctor</Table.TableHead>
          <Table.TableHead>Severity</Table.TableHead>
          <Table.TableHead>Is Urgent</Table.TableHead>

          <Table.TableHead>Actions</Table.TableHead>
        </Table.TableRow>
      </Table.TableHeader>
      <Table.TableBody>
        {#each filteredNotes as note}
          <Table.TableRow>
            <Table.TableCell>
              <a href="/notes/{note.id}">{formatDate(note.createdAt)}</a>
            </Table.TableCell>
            <Table.TableCell>
              <a href="/notes/{note.id}">{getPatientName(note.patientName)}</a>
            </Table.TableCell>
            <Table.TableCell>
              <a href="/notes/{note.id}">{note.department}</a>
            </Table.TableCell>
            <Table.TableCell>
              <a href="/notes/{note.id}">{getDoctorName(note.attendingDoctor)}</a>
            </Table.TableCell>
            <Table.TableCell>
              <a href="/notes/{note.id}">{note.severity}</a>
            </Table.TableCell>
            <Table.TableCell>
              <a href="/notes/{note.id}">
                {#if note.isUrgent}
                  <span class="text-red-600 font-semibold">Yes</span>
                {:else}
                  <span>No</span>
                {/if}
              </a>
            </Table.TableCell>
            <Table.TableCell>
              <DataTableActions id={note.id} patientId={note.patientId} />
            </Table.TableCell>
          </Table.TableRow>
        {/each}
      </Table.TableBody>
    </Table.Table>
  </div>
{/if}
