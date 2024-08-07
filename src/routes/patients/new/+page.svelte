<script lang="ts">
    import PatientForm from "../components/patient-form.svelte";
    let create = true;
    let selectedPatient = null;
</script>




    <h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">Create a Patient</h1>
    <PatientForm {create} {selectedPatient} />

