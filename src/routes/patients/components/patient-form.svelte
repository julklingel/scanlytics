<script lang="ts">
  import DoctorCombobox from "./doctor-combobox.svelte";

  import { Label } from "$lib/components/ui/label/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import { toast } from "svelte-sonner";
  import { invoke } from "@tauri-apps/api/core";
  import ErrorMsg from "../../components/ui/errormodal.svelte";
  import { goto } from "$app/navigation";

  import CalendarIcon from "lucide-svelte/icons/calendar";
  import {
    DateFormatter,
    type DateValue,
    getLocalTimeZone,
  } from "@internationalized/date";
  import { cn } from "$lib/utils.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { UserStore } from "../../../stores/User";

  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });

  export let create = true;
  export let selectedPatient: any = null;
  export const selectedDoc: string = "";

  let errorTitle: string | null | never = "";
  let errorDescription: string | null | never = "";

  let name: string = selectedPatient ? selectedPatient.name : "";
  let gender: string = selectedPatient ? selectedPatient.gender : "";;
  let contactNumber: string = selectedPatient ? selectedPatient.contact_number : "";
  let address: string = selectedPatient ? selectedPatient.address : "";
  let primaryDoctorId: string = selectedPatient ? selectedPatient?.primary_doctor.id.String : "";
  
  let value: DateValue | undefined = undefined;

  async function handleSubmit() {
    const formData = {
      name,
      date_of_birth: value
        ? value.toDate(getLocalTimeZone()).toISOString()
        : null,
      gender,
      contact_number: contactNumber,
      address,
      primary_doctor_id: primaryDoctorId,
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
  class="p-6 bg-white rounded-lg shadow-md"
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
              !value && "text-muted-foreground",
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
      <Label for="gender" class="block text-sm font-medium text-gray-700"
        >Gender</Label
      >
      <select
        id="gender"
        bind:value={gender}
        required
        class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
      >
        <option value="" disabled selected>Select gender</option>
        <option value="male">Male</option>
        <option value="female">Female</option>
        <option value="other">Other</option>
      </select>
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
