<script lang="ts">
    import NoteForm from "../components/note-form.svelte";
    import { patientNotes } from "../../../stores/patientNote";
    import { page } from "$app/stores";

    let create = false;

    $: selectedNote = $patientNotes.find(
        (note) => note.id === $page.params.slug,
    );

</script>

<div class="container mx-auto py-10">
    <h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
        Update Note {selectedNote?.patientName}
    </h1>
    <NoteForm {create} {selectedNote}/>
</div>
