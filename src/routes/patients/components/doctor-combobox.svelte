<script lang="ts">
  import Check from "lucide-svelte/icons/check";
  import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  import { cn } from "$lib/utils.js";
  import Label from "$lib/components/ui/label/label.svelte";

  import { UserStore } from "../../../stores/User";

  export let selectedDoctorId = "";

  $: doctors = $UserStore
    .map((d) => ({
      label: d.name,
      value: d.id,
    }));

  let open = false;

  $: selectedValue = doctors.find((f) => f.value === selectedDoctorId)?.label ?? "Select a doctor...";

  
  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }
</script>



<div class="flex flex-col gap-2 ">
  <Label for="doctor" class="">Doctor</Label>
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
        <Command.Input placeholder="Search doctors..." />
        <Command.Empty>No doctors found.</Command.Empty>
        <Command.Group>
          {#each doctors as doctor}
            <Command.Item
              value={doctor.value}
              onSelect={function (currentValue) {
                selectedDoctorId = currentValue;
                closeAndFocusTrigger(ids.trigger);
              }}
            >
              <Check
              class={cn(
                "mr-2 h-4 w-4",
                selectedDoctorId !== doctor.value && "text-transparent"
              )}
              />
              {doctor.label}
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.Root>
    </Popover.Content>
  </Popover.Root>
</div>
