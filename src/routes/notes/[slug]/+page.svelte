<script lang="ts">
    import NoteForm from "../components/note-form.svelte";
    import { patientNotes } from "../../../stores/PatientNote";
    import { page } from "$app/stores";
    import { PatientStore } from "../../../stores/Patient";

    let create = false;

    $: selectedNote = $patientNotes.find(
        (note) => note.id === $page.params.slug,
    );

    function getPatientName(patientId: string) {
    let patientName = "No patient assigned";
    PatientStore.subscribe(patients => {
      const patient = patients.find((p: any) => p.id === patientId);
      if (patient) {
        patientName = patient.name;
      }
    })();
    return patientName;
  }



</script>

<div class="container mx-auto py-10">
    <h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
        Update Note {getPatientName(selectedNote?.patientName)}
    </h1>
    <NoteForm {create} {selectedNote}/>
</div>
