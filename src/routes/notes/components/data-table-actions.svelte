<script lang="ts">
    import Ellipsis from "lucide-svelte/icons/ellipsis";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import { invoke } from "@tauri-apps/api/core";
    import { PatientNotesStore } from "../../../stores/PatientNote";
    import { toast } from "svelte-sonner";


   
    export let id: string;
    export let patientId: string;



    function deleteNote() {
      try {
        invoke("delete_patient_note", { id });
        PatientNotesStore.update((notes) => notes.filter((note) => note.id !== id));
        toast("Note deleted successfully");

      } catch (error) {
        console.error(error);
        toast(`Something went wrong: ${error}`);
      }
      
    }
</script>
   
<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
      <Button
        variant="ghost"
        builders={[builder]}
        size="icon"
        class="relative h-8 w-8 p-0"
      >
        <span class="sr-only">Open menu</span>
        <Ellipsis class="h-4 w-4" />
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
      <DropdownMenu.Group>
        <DropdownMenu.Label>Actions</DropdownMenu.Label>
        <DropdownMenu.Item on:click={() => navigator.clipboard.writeText(id)}>
          Copy note ID
        </DropdownMenu.Item>
        <DropdownMenu.Item on:click={() => navigator.clipboard.writeText(patientId)}>
          Copy patient ID
        </DropdownMenu.Item>
      </DropdownMenu.Group>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>View patient details</DropdownMenu.Item>
      <DropdownMenu.Item>Edit note</DropdownMenu.Item>
      <DropdownMenu.Item>View full medical history</DropdownMenu.Item>
      <DropdownMenu.Item>Schedule follow-up</DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item class="text-red-600">Mark as urgent</DropdownMenu.Item>
      <DropdownMenu.Item on:click={deleteNote} class="text-red-600">Delete Note</DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>