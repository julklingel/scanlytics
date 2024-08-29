<script lang="ts">
  import { PatientStore } from "../../../stores/Patient";
  import { createEventDispatcher } from "svelte";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  export let patient_id = "";
  const dispatch = createEventDispatcher();

  $: patients = $PatientStore.map((p) => ({
    label: p.name,
    value: p.id,
  }));

  let searchTerm = "";
  let filteredPatients = patients;
  let isInputFocused = false;

  $: {
    filteredPatients = patients.filter((patient) =>
      patient.label.toLowerCase().includes(searchTerm.toLowerCase()),
    );
  }

  function handleSelect(patient: any) {
    patient_id = patient.value.String;
    dispatch("select", { id: patient.value });
    isInputFocused = false;
    searchTerm = patient.label;
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
    placeholder="Search for a patient"
    bind:value={searchTerm}
    on:focus={handleInputFocus}
    on:blur={handleInputBlur}
  />
  {#if isInputFocused}
    {#if filteredPatients.length > 0}
      <ul class="flex flex-col absolute w-full bg-white border border-gray-300 mt-1 max-h-60 overflow-y-auto z-10">
        {#each filteredPatients as patient}
          <Button variant="ghost" on:click={() => handleSelect(patient)} class="text-left p-2 hover:bg-gray-100">
            {patient.label}
          </Button>
        {/each}
      </ul>
    {:else}
      <p class="text-gray-500 mt-2 absolute w-full bg-white border border-gray-300 p-2 z-10">No patients found</p>
    {/if}
  {/if}
</div>