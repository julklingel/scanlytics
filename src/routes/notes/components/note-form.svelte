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

  let value: DateValue | undefined = undefined;

  let errorTitle: string | null | never = "";
  let errorDescription: string | null | never = "";

  let patient: string = selectedNote ? selectedNote.patient : "";
  let symptoms: string = selectedNote ? selectedNote.symptoms : "";
  let diagnosis: string = selectedNote ? selectedNote.diagnosis : "";
  let treatment: string = selectedNote ? selectedNote.treatment : "";
  let isUrgent: boolean = selectedNote ? selectedNote.isUrgent : false;
  let department: string = selectedNote ? selectedNote.department : "";
  let attendingDoctor: string = selectedNote
    ? selectedNote.attendingDoctor
    : "";
  let severity: string = selectedNote ? selectedNote.severity : "";

  async function handleSubmit() {
    const formData = {
      patient,
      symptoms,
      diagnosis,
      treatment,
      follow_up_date: value
        ? df.format(value.toDate(getLocalTimeZone()))
        : null,
      severity,
      is_urgent: isUrgent,
      department,
      attending_doctor: attendingDoctor,
    };

    if (create) {
      try {
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
  class="py-6 bg-white rounded-lg shadow-md"
  on:submit|preventDefault={handleSubmit}
>
  <div class="grid grid-cols-1 gap-7">
    <div class=" ">
      <div class="mb-4">
      <Label for="patientName" class="block text-sm font-medium text-gray-700"
        >Patient
      </Label>
      <PatientCombobox
        bind:selectedpatientId={patientId}
        bind:selectedpatientName={patientName}
      />
    </div>

      <div class="mb-4">
        <Label for="symptoms" class="block text-sm font-medium text-gray-700"
          >Symptoms</Label
        >
        <Textarea
          name="symptoms"
          bind:value={symptoms}
          placeholder="Type your message here."
        />
      </div>

      <div class="mb-4">
        <Label for="diagnosis" class="block text-sm font-medium text-gray-700"
          >Diagnosis</Label
        >
        <Textarea
          name="diagnosis"
          bind:value={diagnosis}
          placeholder="Type your message here."
        />
      </div>

      <div class="mb-4">
        <Label for="treatment" class="block text-sm font-medium text-gray-700"
          >Treatment</Label
        >
        <Textarea
          name="treatment"
          bind:value={treatment}
          placeholder="Type your message here."
        />
      </div>


      <div class="mb-4">
        <label for="severity" class="block text-sm font-medium text-gray-700"
          >Severity</label
        >
        <select
          id="severity"
          name="severity"
          bind:value={severity}
          class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
        >
          <option value="" disabled>Select a Severity</option>
          <option value="low">Low</option>
          <option value="medium">Medium</option>
          <option value="high">High</option>
        </select>
      </div>
      <div class="flex items-center space-x-2 mb-4">
        <Switch
          id="urgent-mode"
          bind:checked={isUrgent}
          class="urgent-switch"
        />
        <Label for="urgent-mode" class="text-sm font-medium text-gray-900"
          >Is Urgent</Label
        >
      </div>

      <div class="mb-4">
        <Label for="department" class="block text-sm font-medium text-gray-700"
          >Department</Label
        >
        <Input
          type="text"
          id="department"
          name="department"
          bind:value={department}
          class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        />
      </div>

      <div>
        <Label
          for="attendingDoctor"
          class="block text-sm font-medium text-gray-700"
          >Attending Doctor</Label
        >
        <DoctorCombobox bind:selectedDoctorId={attendingDoctor} />
        
      </div>
    </div>

    {#if errorDescription}
      <ErrorMsg {errorTitle} {errorDescription} />
    {/if}

    <div class="mt-6">
      <Button type="submit" class="w-full px-4 py-2 font-semibold">
        {#if create}
          Create
        {:else}
          Update
        {/if}
      </Button>
    </div>
  </div>
</form>

<style>
  :global(.urgent-switch[data-state="checked"]) {
    background-color: rgb(194, 2, 2) !important;
  }
</style>
