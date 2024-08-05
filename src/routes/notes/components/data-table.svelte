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
        patientName: note.patient_name,
        patientId: note.patient_id,
        symptoms: note.symptoms,
        diagnosis: note.diagnosis,
        treatment: note.treatment,
        createdAt: note.created_at,
        updatedAt: note.updated_at,
        severity: note.severity,
        isUrgent: note.is_urgent,
        department: note.department,
        attendingDoctor: note.attending_doctor,
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
  });

  $: filteredNotes = $filterValue
    ? $patientNotes.filter((note) =>
        note.patientName.toLowerCase().includes($filterValue.toLowerCase()),
      )
    : $patientNotes;

  

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
          <Table.TableHead>Patient ID</Table.TableHead>
          <Table.TableHead>Department</Table.TableHead>
          <Table.TableHead>Attending Doctor</Table.TableHead>
          <Table.TableHead>Severity</Table.TableHead>
          <Table.TableHead>Is Urgent</Table.TableHead>

          <Table.TableHead>Actions</Table.TableHead>
        </Table.TableRow>
      </Table.TableHeader>
      <Table.TableBody>
        {#each filteredNotes as note}
          <Table.TableRow >
            <Table.TableCell on:click={() => handleNoteView(note.id)}>{formatDate(note.createdAt)}</Table.TableCell>
            <Table.TableCell on:click={() => handleNoteView(note.id)}>{note.patientName}</Table.TableCell>
            <Table.TableCell on:click={() => handleNoteView(note.id)}>{note.patientId}</Table.TableCell>
            <Table.TableCell on:click={() => handleNoteView(note.id)}>{note.department}</Table.TableCell>
            <Table.TableCell on:click={() => handleNoteView(note.id)}>{note.attendingDoctor}</Table.TableCell>
            <Table.TableCell on:click={() => handleNoteView(note.id)}>{note.severity}</Table.TableCell>
            <Table.TableCell on:click={() => handleNoteView(note.id)}>
              {#if note.isUrgent}
                <span class="text-red-600 font-semibold">Yes</span>
              {:else}
                <span>No</span>
              {/if}
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
