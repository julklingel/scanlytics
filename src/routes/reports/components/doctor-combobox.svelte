<script lang="ts">
import { UserStore } from "../../../stores/User";
  import { createEventDispatcher } from "svelte";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  export let selectedDoctorId = "";

  const dispatch = createEventDispatcher();

  $: doctors = $UserStore.map((d) => ({
    label: d.name,
    value: d.id,
  }));

  let searchTerm = "";
  let filteredDoctors = doctors;
  let isInputFocused = false;

  $: {
    filteredDoctors = doctors.filter((doctor) =>
      doctor.label.toLowerCase().includes(searchTerm.toLowerCase()),
    );
  }

  function handleSelect(doctor: any) {
    selectedDoctorId = doctor.value.String;
    dispatch("select", { id: doctor.value });
    isInputFocused = false;
    searchTerm = doctor.label;
  }

  function handleInputFocus() {
    isInputFocused = true;
  }

  function handleInputBlur() {
    setTimeout(() => {
      isInputFocused = false;
    }, 200);
  }
</script>

<div class="relative">
  <Input
    type="text"
    placeholder="Search for a doctor"
    bind:value={searchTerm}
    on:focus={handleInputFocus}
    on:blur={handleInputBlur}
  />
  {#if isInputFocused}
    {#if filteredDoctors.length > 0}
      <ul class="flex flex-col absolute w-full bg-white border border-gray-300 mt-1 max-h-60 overflow-y-auto z-10">
        {#each filteredDoctors as doctor}
          <Button variant="ghost" on:click={() => handleSelect(doctor)} class="text-left p-2 hover:bg-gray-100">
            {doctor.label}
          </Button>
        {/each}
      </ul>
    {:else}
      <p class="text-gray-500 mt-2 absolute w-full bg-white border border-gray-300 p-2 z-10">No doctors found</p>
    {/if}
  {/if}
</div>