<script lang="ts">
    import NoteForm from "../components/note-form.svelte";
    import { PatientNotesStore } from "../../../stores/PatientNote";

    let create = true;
    let selectedPatient = null;
</script>

<h1 class="my-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
    Erstelle eine Notiz
</h1>

<NoteForm create selectedNote />
