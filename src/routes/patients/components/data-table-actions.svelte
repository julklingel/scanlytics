<script lang="ts">
    import Ellipsis from "lucide-svelte/icons/ellipsis";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import { invoke } from "@tauri-apps/api/core";
    import { PatientStore } from "../../../stores/patient";
    import { toast } from "svelte-sonner";


   
    export let id: string;



    function deletePatient() {
      try {
        invoke("delete_patient", { id });
        PatientStore.update((p:any) => p.filter((p:any) => p.id !== id));
        toast("Patient deleted successfully");

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
          Copy patient ID
        </DropdownMenu.Item>
      </DropdownMenu.Group>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>View patient details</DropdownMenu.Item>
      <DropdownMenu.Item>Edit Patient</DropdownMenu.Item>
      <DropdownMenu.Item>View full medical history</DropdownMenu.Item>
      <DropdownMenu.Separator />
    
      <DropdownMenu.Item on:click={deletePatient} class="text-red-600">Delete Patient</DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>