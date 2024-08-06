<script lang="ts">
  import Check from "lucide-svelte/icons/check";
  import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  import { cn } from "$lib/utils.js";
  import Label from "$lib/components/ui/label/label.svelte";

  import { PatientStore } from "../../../stores/Patient";

  export let selectedpatientId = "";
  export let selectedpatientName = "";

  $: patients = $PatientStore
    .map((d) => ({
      label: d.name,
      value: d.id,
    }));

  let open = false;

  $: selectedValue = patients.find((f) => f.value === selectedpatientId)?.label ?? "Select a patient...";
  $: selectedpatientName = patients.find((f) => f.value === selectedpatientId)?.label ?? "";
  
  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }
</script>



<div class="flex flex-col gap-2 ">

  <Popover.Root bind:open let:ids>
    <Popover.Trigger asChild let:builder>
      <Button
        builders={[builder]}
        variant="outline"
        role="combobox"
        aria-expanded={open}
        class=" justify-between"
      >
        {selectedValue}
        <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    </Popover.Trigger>
    <Popover.Content class=" p-0">
      <Command.Root>
        <Command.Input placeholder="Search patients..." />
        <Command.Empty>No patients found.</Command.Empty>
        <Command.Group>
          {#each patients as patient}
            <Command.Item
              value={patient.value}
              onSelect={function (currentValue) {
                selectedpatientId = currentValue;
                closeAndFocusTrigger(ids.trigger);
              }}
            >
              <Check
              class={cn(
                "mr-2 h-4 w-4",
                selectedpatientId !== patient.value && "text-transparent"
              )}
              />
              {patient.label}
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
</div>
