<script lang="ts">
    import Ellipsis from "lucide-svelte/icons/ellipsis";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
   
    export let id: string;
    export let patientId: string;
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
    </DropdownMenu.Content>
</DropdownMenu.Root>