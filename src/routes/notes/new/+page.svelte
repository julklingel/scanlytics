<script lang="ts">
    import NoteForm from "../components/note-form.svelte";
    let create = true;
    let selectedNote = null;
</script>



<div class="container mx-auto py-10">
    <h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">Create a Note</h1>
    <NoteForm {create} {selectedNote} />
</div>
