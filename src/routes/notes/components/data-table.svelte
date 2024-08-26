<script lang="ts">
  import DataTableActions from "./data-table-actions.svelte";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import PlusIcon from "lucide-svelte/icons/plus";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { writable, derived } from "svelte/store";
  import InfoMsg from "../../components/ui/infomodal.svelte";
  import { PatientNotesStore } from "../../../stores/PatientNote";
  import { getPatientNotes } from "../api/patients-note-data";

  let filteredPatientNotes: any;
  const infoTitle: string | null | never = "No patient notes found";
  const infoDescription: string | null | never =
    "There are no patient notes available. Please add a new note.";
  let dataAvailable: boolean = false;
  const filterValue = writable("");

  const isPatientNotesStoreEmpty = derived(
    PatientNotesStore,
    ($PatientNotesStore) => $PatientNotesStore.length === 0,
  );
  $: dataAvailable = !$isPatientNotesStoreEmpty;

  onMount(async () => {
    try {
      await getPatientNotes();
    } catch (error) {
      console.error(error);
    }
  });

  $: filteredPatientNotes = $filterValue
    ? $PatientNotesStore.filter(
        (note) =>
          (note.patient?.name || "")
            .toLowerCase()
            .includes($filterValue.toLowerCase()) ||
          (note.symptoms || "")
            .toLowerCase()
            .includes($filterValue.toLowerCase()) ||
          (note.diagnosis || "")
            .toLowerCase()
            .includes($filterValue.toLowerCase()),
      )
    : $PatientNotesStore;

  
  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }

  function handleCreateNewPatientNote() {
    goto("/notes/new");
  }


  function handleNoteView(id: string) {
    goto(`/notes/${id}`);
  }
</script>

{#if !dataAvailable}
  <div class="flex flex-col gap-4">
    <InfoMsg {infoTitle} {infoDescription} />
    <Button on:click={handleCreateNewPatientNote}>
      <PlusIcon size={16} />
      <span>Add New Note</span>
    </Button>
  </div>
{:else}
  <div>
    <div class="flex gap-4 mb-2">
      <Button on:click={handleCreateNewPatientNote}>
        <PlusIcon size={16} />
        <span>Add New Note</span>
      </Button>
      <Input
        class="max-w-sm"
        placeholder="Filter notes..."
        type="text"
        bind:value={$filterValue}
      />
    </div>
    <Table.Table>
      <Table.TableHeader>
        <Table.TableRow>
          <Table.TableHead>Patient</Table.TableHead>
          <Table.TableHead>Doctor</Table.TableHead>
          <Table.TableHead>Diagnosis</Table.TableHead>
          <Table.TableHead>Urgent</Table.TableHead>
          <Table.TableHead>Created At</Table.TableHead>
          <Table.TableHead>Actions</Table.TableHead>
        </Table.TableRow>
      </Table.TableHeader>
      <Table.TableBody>
        {#each filteredPatientNotes as note}
          <Table.TableRow>
            <Table.TableCell on:click={() => handleNoteView(note.id.id.String)}
              >{note.patient?.name || "N/A"}</Table.TableCell
            >
            <Table.TableCell on:click={() => handleNoteView(note.id.id.String)}
              >{note.userOwner?.name || "N/A"}</Table.TableCell
            >
            <Table.TableCell on:click={() => handleNoteView(note.id.id.String)}
              >{note.diagnosis || "N/A"}</Table.TableCell
            ><Table.TableCell on:click={() => handleNoteView(note.id.id.String)}
              >{note.is_urgent ? "Yes" : "No"}</Table.TableCell
            >
            <Table.TableCell on:click={() => handleNoteView(note.id.id.String)}
              >{formatDate(note.createdAt)}</Table.TableCell
            >
            <Table.TableCell>
              <DataTableActions id={note.id} patientId={note.patient?.id} />
            </Table.TableCell>
          </Table.TableRow>
        {/each}
      </Table.TableBody>
    </Table.Table>
  </div>
{/if}
