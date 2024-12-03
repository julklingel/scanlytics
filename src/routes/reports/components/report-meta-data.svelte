<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import PatientCombobox from "../components/patient-combobox.svelte";
  import DoctorCombobox from "../components/doctor-combobox.svelte";

  export let patient_id: string;
  export let user_owner: string;
</script>

<div class="flex gap-60">
  <div class="">
    <Label
    for="attendingDoctor"
    class="block text-sm font-medium text-gray-700">Attending Doctor</Label
    >
    <DoctorCombobox bind:selectedDoctorId={user_owner} />
  </div>
  <div class="">
    <Label for="patientName" class="block text-sm font-medium text-gray-700"
      >Patient</Label
    >
    <PatientCombobox bind:patient_id />
  </div>
</div>
