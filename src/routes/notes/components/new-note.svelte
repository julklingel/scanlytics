<script lang="ts">
  import Card from "../../components/ui/card.svelte";
  import { Label } from "$lib/components/ui/label/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import Textarea from "$lib/components/ui/textarea/textarea.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { Switch } from "$lib/components/ui/switch/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { invoke } from "@tauri-apps/api/core";

  import CalendarIcon from "lucide-svelte/icons/calendar";
  import {
    DateFormatter,
    type DateValue,
    getLocalTimeZone,
  } from "@internationalized/date";
  import { cn } from "$lib/utils.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";

  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });

  let value: DateValue | undefined = undefined;

  const severitylvls = [
    { value: "low", label: "Low" },
    { value: "Medium", label: "Medium" },
    { value: "High", label: "High" },
  ];

  let patientName: string = "";
  let patientId: string = "";
  let symptoms: string = "";
  let diagnosis: string = "";
  let treatment: string = "";
  let isUrgent: boolean = false;
  let department: string = "";
  let attendingDoctor: string = "";
  let severity: string = "";

  async function handleSubmit() {
    const formData = {
      patient_name: patientName,
      patient_id: patientId,
      symptoms,
      diagnosis,
      treatment,
      follow_up_date: value
        ? df.format(value.toDate(getLocalTimeZone()))
        : null,
      severity,
      is_urgent: isUrgent,
      department,
      attending_doctor: attendingDoctor
    };

    try {
      const response = await invoke("create_patient_note", {
        patientNoteRequest: formData,
      });
      console.log("Response from backend:", response);
      alert(JSON.stringify(response));
    } catch (error) {
      console.error("Error submitting form:", error);
      alert("ERROR");
      alert(JSON.stringify(error));
    }
  }
</script>

<form
  id="patientNoteForm"
  class="p-6 bg-white rounded-lg shadow-md"
  on:submit|preventDefault={handleSubmit}
>
  <div class="grid grid-cols-1 gap-6">
    <div>
      <Label for="patientName" class="block text-sm font-medium text-gray-700"
        >Patient Name</Label
      >
      <Input
        type="text"
        id="patientName"
        name="patientName"
        required
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>

    <div>
      <Label for="patientId" class="block text-sm font-medium text-gray-700"
        >Patient ID</Label
      >
      <Input
        type="text"
        id="patientId"
        name="patientId"
        required
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>

    <div>
      <Label for="symptoms" class="block text-sm font-medium text-gray-700"
        >Symptoms</Label
      >
      <Textarea placeholder="Type your message here." />
    </div>

    <div>
      <Label for="diagnosis" class="block text-sm font-medium text-gray-700"
        >Diagnosis</Label
      >
      <Textarea placeholder="Type your message here." />
    </div>

    <div>
      <Label for="treatment" class="block text-sm font-medium text-gray-700"
        >Treatment</Label
      >
      <Textarea placeholder="Type your message here." />
    </div>

    <div class=" ">
      <Label for="followupDate" class="block text-sm font-medium text-gray-700"
        >Follow Up Date</Label
      >
      <Popover.Root openFocus>
        <Popover.Trigger asChild let:builder>
          <Button
            variant="outline"
            class={cn(
              "w-[280px] justify-start text-left font-normal",
              !value && "text-muted-foreground",
            )}
            builders={[builder]}
          >
            <CalendarIcon class="mr-2 h-4 w-4" />
            {value
              ? df.format(value.toDate(getLocalTimeZone()))
              : "Select a date"}
          </Button>
        </Popover.Trigger>
        <Popover.Content class="w-auto p-0">
          <Calendar bind:value initialFocus />
        </Popover.Content>
      </Popover.Root>
    </div>

    <div>
      <Label for="severity" class="block text-sm font-medium text-gray-700"
        >Severity</Label
      >
      <Select.Root portal={null}>
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Select a Severity" />
        </Select.Trigger>
        <Select.Content>
          <Select.Group>
            <Select.Label>Severity</Select.Label>
            {#each severitylvls as s}
              <Select.Item value={s.value} label={s.label}
                >{s.label}</Select.Item
              >
            {/each}
          </Select.Group>
        </Select.Content>
        <Select.Input name="pick" />
      </Select.Root>
    </div>

    <div class="flex items-center space-x-2">
      <Switch id="urgent-mode" class="urgent-switch" />
      <Label for="urgent-mode" class="text-sm font-medium text-gray-900"
        >Is Urgent</Label
      >
    </div>

    <div>
      <Label for="department" class="block text-sm font-medium text-gray-700"
        >Department</Label
      >
      <Input
        type="text"
        id="department"
        name="department"
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>

    <div>
      <Label
        for="attendingDoctor"
        class="block text-sm font-medium text-gray-700">Attending Doctor</Label
      >
      <Input
        type="text"
        id="attendingDoctor"
        name="attendingDoctor"
        required
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>
  </div>

  <div class="mt-6">
    <Button type="submit" class="w-full px-4 py-2 font-semibold">Submit</Button>
  </div>
</form>

<style>
  :global(.urgent-switch[data-state="checked"]) {
    background-color: rgb(194, 2, 2) !important;
  }
</style>
