<script lang="ts">
    import PatientForm from "../components/patient-form.svelte";
    import { PatientStore } from "../../../stores/Patient";
    import { page } from "$app/stores";

    $: selectedPatient = $PatientStore.find(
        (patient) => patient.id === $page.params.slug,
    );
</script>

<h1 class="my-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
    Überblick Patient {selectedPatient?.name}
</h1>
