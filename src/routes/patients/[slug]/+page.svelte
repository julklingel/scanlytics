<script lang="ts">
    import  PatientForm  from "../components/patient-form.svelte";
    import { PatientStore } from "../../../stores/patient";
    import { page } from "$app/stores";

    let create = false;

    $: selectedPatient = $PatientStore.find(
        (patient) => patient.id === $page.params.slug,
    );

</script>

<div class="container mx-auto py-10">
    <h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
        Update Patient {selectedPatient?.name}
    </h1>
    <PatientForm {create} {selectedPatient}/>
</div>
