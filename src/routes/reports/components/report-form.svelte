<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import Textarea from "$lib/components/ui/textarea/textarea.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { toast } from "svelte-sonner";
  import { invoke } from "@tauri-apps/api/core";
  import PatientCombobox from "./patient-combobox.svelte";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { getPatients } from "../api/patient-data";
  import ErrorMsg from "../../components/ui/errormodal.svelte";
  import { DateFormatter } from "@internationalized/date";

  export let create: boolean;
  export let selectedReport: any;

  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });

  let errorTitle: string | null | never = "";
  let errorDescription: string | null | never = "";

  let patient_id: string = selectedReport ? selectedReport.patient.id : "";
  let body_type: string = selectedReport ? selectedReport.body_type : "";
  let condition: string = selectedReport ? selectedReport.condition : "";
  let report_text: string = selectedReport ? selectedReport.report_text : "";

  onMount(async () => {
    try {
      await getPatients();
    } catch (error) {
      console.error(error);
    }
  });

  async function handleSubmit() {
    const formData = {
      patient_id,
      body_type,
      condition,
      report_text
    };

    if (create) {
      try {
        console.log("formData", formData);

        const response = await invoke("create_report", {
          reportRequest: JSON.stringify(formData),
          success: true,
        });
        goto("/reports");
        toast(`Report created successfully!`);

        errorDescription = null;
      } catch (error) {
        console.error("Error submitting form:", error);
        errorDescription =
          error instanceof Error ? error.message : String(error);
      }
    } else {
      try {
        const response = await invoke("update_report", {
          id: selectedReport.id,
          reportRequest: JSON.stringify(formData),
          success: true,
        });
        goto("/reports");
        toast(`Report updated successfully!`);

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
  id="reportForm"
  class="py-6 bg-white rounded-lg shadow-md p-6"
  on:submit|preventDefault={handleSubmit}
>
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
    <div class="col-span-2 sm:col-span-4">
      <Label for="patientName" class="block text-sm font-medium text-gray-700">Patient</Label>
      <PatientCombobox bind:patient_id={patient_id} />
    </div>
    <div class="col-span-2 sm:col-span-2">
      <Label for="bodyType" class="block text-sm font-medium text-gray-700">Body Type</Label>
      <Input type="text" id="bodyType" bind:value={body_type} placeholder="Enter body type" />
    </div>
    <div class="col-span-2 sm:col-span-2">
      <Label for="condition" class="block text-sm font-medium text-gray-700">Condition</Label>
      <Input type="text" id="condition" bind:value={condition} placeholder="Enter condition" />
    </div>
  </div>

  <div class="mb-6">
    <Label for="reportText" class="block text-lg font-bold mb-2">Report Text</Label>
    <Textarea
      name="reportText"
      bind:value={report_text}
      placeholder="Enter the report text"
      class="w-full h-64"
    />
  </div>

  {#if errorDescription}
    <ErrorMsg {errorTitle} {errorDescription} />
  {/if}

  <div class="mt-6">
    <Button type="submit" class="w-full px-4 py-2 font-semibold">
      {create ? 'Create Report' : 'Update Report'}
    </Button>
  </div>
</form>