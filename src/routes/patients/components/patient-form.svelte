<script lang="ts">
  import DoctorCombobox from "./doctor-combobox.svelte";

  import { Label } from "$lib/components/ui/label/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { toast } from "svelte-sonner";
  import { invoke } from "@tauri-apps/api/core";
  import ErrorMsg from "../../components/ui/errormodal.svelte";
  import { goto } from "$app/navigation";
  import * as Select from "$lib/components/ui/select";

  import CalendarIcon from "lucide-svelte/icons/calendar";
  import {
    DateFormatter,
    type DateValue,
    getLocalTimeZone,
  } from "@internationalized/date";
  import { cn } from "$lib/utils.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { getUsers } from "../api/user-data";
  import { onMount } from "svelte";

  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });

  export let create = true;
  export let selectedPatient: any = null;
  export const selectedDoc: string = "";

  let errorTitle: string | null | never = "";
  let errorDescription: string | null | never = "";

  let name: string = selectedPatient ? selectedPatient.name : "";
  let gender: string = selectedPatient ? selectedPatient.gender : "";
  let contactNumber: string = selectedPatient
    ? selectedPatient.contact_number
    : "";
  let address: string = selectedPatient ? selectedPatient.address : "";
  let primaryDoctorId: string = selectedPatient
    ? selectedPatient?.primary_doctor.id.String
    : "";

  let value: DateValue | undefined = undefined;

  let selected: any;
  $: if (selected) {
    gender = selected.value;
  }


  onMount(async () => {
    try {
      await getUsers();
    } catch (error) {
      console.error(error);
    }
  });

  async function handleSubmit() {
    const formData = {
      name,
      date_of_birth: value
        ? value.toDate(getLocalTimeZone()).toISOString()
        : null,
      gender,
      contact_number: contactNumber,
      address,
      primary_doctor: (primaryDoctorId as any).String,
    };

    if (create) {
      try {
        const response = await invoke("create_patient", {
          patientRequest: JSON.stringify(formData),
        });

        goto("/patients");
        toast(`Patient created successfully!`);
        errorDescription = null;
      } catch (error) {
        console.error("Error submitting form:", error);
        errorDescription =
          error instanceof Error ? error.message : String(error);
      }
    } else {
      try {
        const response = await invoke("update_patient", {
          patientRequest: JSON.stringify(formData),
          id: selectedPatient.id,
        });
        goto(`/patients`);
        toast(`Patient updated successfully!`);
        errorDescription = null;
      } catch (error) {
        console.error("Error submitting form:", error);
        errorDescription =
          error instanceof Error ? error.message : String(error);
      }
    }
  }
</script>

<form
  id="patientForm"
  class="py-6 bg-white rounded-lg shadow-md p-6"
  on:submit|preventDefault={handleSubmit}
>
  <div class="grid grid-cols-1 gap-6">
    <div>
      <Label for="name" class="block text-sm font-medium text-gray-700"
        >Name</Label
      >
      <Input
        type="text"
        id="name"
        bind:value={name}
        required
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>

    <div>
      <Label for="dateOfBirth" class="block text-sm font-medium text-gray-700"
        >Date of Birth</Label
      >
      <Popover.Root>
        <Popover.Trigger asChild let:builder>
          <Button
            variant="outline"
            class={cn(
              "w-[280px] justify-start text-left font-normal",
              !value && "text-muted-foreground"
            )}
            builders={[builder]}
          >
            <CalendarIcon class="mr-2 h-4 w-4" />
            {value
              ? df.format(value.toDate(getLocalTimeZone()))
              : "Pick a date"}
          </Button>
        </Popover.Trigger>
        <Popover.Content class="w-auto p-0">
          <Calendar bind:value initialFocus />
        </Popover.Content>
      </Popover.Root>
    </div>

    <div>
      <Label> Gender</Label>
      <Select.Root bind:selected>
        <Select.Trigger>
          <Select.Value placeholder="Select your Gender" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="male">Male</Select.Item>
          <Select.Item value="female">Female</Select.Item>
          <Select.Item value="other">Other</Select.Item>
        </Select.Content>
      </Select.Root>
    </div>

    <div>
      <Label for="contactNumber" class="block text-sm font-medium text-gray-700"
        >Contact Number</Label
      >
      <Input
        type="tel"
        id="contactNumber"
        bind:value={contactNumber}
        required
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>

    <div>
      <Label for="address" class="block text-sm font-medium text-gray-700"
        >Address</Label
      >
      <Input
        type="text"
        id="address"
        bind:value={address}
        required
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>

    <Label>Attending Doctor</Label>
    <DoctorCombobox bind:selectedDoctorId={primaryDoctorId} />
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
</form>
