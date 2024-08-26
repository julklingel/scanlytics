<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import Textarea from "$lib/components/ui/textarea/textarea.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { toast } from "svelte-sonner";
  import { Switch } from "$lib/components/ui/switch/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import PatientCombobox from "./patient-combobox.svelte";
  import DoctorCombobox from "../../patients/components/doctor-combobox.svelte";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { getPatients } from "../api/patient-data";
  import { getUsers } from "../api/user-data";
  import * as Select from "$lib/components/ui/select";

  import ErrorMsg from "../../components/ui/errormodal.svelte";

  import {
    DateFormatter,
    type DateValue,
    getLocalTimeZone,
  } from "@internationalized/date";

  
    
  export let create: boolean;
  export let selectedNote: any;

  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });

  let selected: any = "null";


  let errorTitle: string | null | never = "";
  let errorDescription: string | null | never = "";

  let patient_id: string = selectedNote ? selectedNote.patient.id : "";
  let symptoms: string = selectedNote ? selectedNote.symptoms : "";
  let diagnosis: string = selectedNote ? selectedNote.diagnosis : "";
  let treatment: string = selectedNote ? selectedNote.treatment : "";
  let isUrgent: boolean = selectedNote ? selectedNote.isUrgent : false;
  let userOwner: string = selectedNote
    ? selectedNote.userOwner
    : "";
  let severity: string =  selectedNote ? selectedNote.severity : "";
  $: severity = selected.value; 


  onMount(async () => {
    try {
      await getPatients();
      await getUsers();
    } catch (error) {
      console.error(error);
    }

  });

  async function handleSubmit() {
    const formData = {
      patient_id,
      symptoms,
      diagnosis,
      treatment,
      severity,
      is_urgent: isUrgent,
      user_owner: userOwner.String,
    };

    if (create) {
      try {
        console.log("formData", formData);

        const response = await invoke("create_patient_note", {
          patientNoteRequest: JSON.stringify(formData),
          success: true,
        });
        goto("/notes");
        toast(`Note created successfully!`);

        errorDescription = null;
      } catch (error) {
        console.error("Error submitting form:", error);
        errorDescription =
          error instanceof Error ? error.message : String(error);
      }
    } else {
      try {
        const response = await invoke("update_patient_note", {
          id: selectedNote.id,
          patientNoteRequest: JSON.stringify(formData),
          success: true,
        });
        goto("/notes");
        toast(`Note updated successfully!`);

        errorDescription = null;
      } catch (error) {
        console.error("Error updating form:", error);
        errorDescription =
          error instanceof Error ? error.message : String(error);
      }
    }
  }
  
  
</script>

<form
  id="patientNoteForm"
  class="py-6 bg-white rounded-lg shadow-md p-6"
  on:submit|preventDefault={handleSubmit}
>
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
    <div class="col-span-2 sm:col-span-4">
      <Label for="patientName" class="block text-sm font-medium text-gray-700">Patient</Label>
      <PatientCombobox bind:patient_id={patient_id} />
    </div>
    <div class="col-span-2 sm:col-span-2">
      <Label for="attendingDoctor" class="block text-sm font-medium text-gray-700">Attending Doctor</Label>
      <DoctorCombobox bind:selectedDoctorId={userOwner} />
    </div>
    <div class="col-span-2 sm:col-span-1">
      <Label for="severity" class="block text-sm font-medium text-gray-700">Severity</Label>
      <Select.Root bind:selected>
        <Select.Trigger class="w-full">
          <Select.Value placeholder="Severity" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="low">Low</Select.Item>
          <Select.Item value="medium">Medium</Select.Item>
          <Select.Item value="high">High</Select.Item>
        </Select.Content>
      </Select.Root>
    </div>

    <div class="col-span-2 sm:col-span-1 flex items-center pt-4">
      <Switch id="urgent-mode" bind:checked={isUrgent} class="urgent-switch mr-2" />
      <Label for="urgent-mode" class="text-sm font-medium text-gray-900">Is Urgent</Label>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div class="bg-yellow-50 rounded-lg p-4 shadow">
      <Label for="symptoms" class="block text-lg font-bold mb-2">Symptoms</Label>
      <Textarea
        name="symptoms"
        bind:value={symptoms}
        placeholder="Describe the symptoms"
        class="w-full bg-yellow-50"
      />
    </div>
    <div class="bg-blue-50 rounded-lg p-4 shadow">
      <Label for="diagnosis" class="block text-lg font-bold mb-2">Diagnosis</Label>
      <Textarea
        name="diagnosis"
        bind:value={diagnosis}
        placeholder="Enter the diagnosis"
        class="w-full bg-blue-50"
      />
    </div>
    <div class="bg-green-50 rounded-lg p-4 shadow">
      <Label for="treatment" class="block text-lg font-bold mb-2">Treatment</Label>
      <Textarea
        name="treatment"
        bind:value={treatment}
        placeholder="Describe the treatment"
        class="w-full bg-green-50 "
      />
    </div>
  </div>

  {#if errorDescription}
    <ErrorMsg {errorTitle} {errorDescription} />
  {/if}

  <div class="mt-6">
    <Button type="submit" class="w-full px-4 py-2 font-semibold">
      {create ? 'Create' : 'Update'}
    </Button>
  </div>
</form>

<style>
  :global(.urgent-switch[data-state="checked"]) {
    background-color: rgb(194, 2, 2) !important;
  }
</style>