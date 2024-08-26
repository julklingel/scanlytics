<script lang="ts">
  import { PatientNotesStore } from "../../../stores/PatientNote";
  import { page } from "$app/stores";
  import NoteForm from "../components/note-form.svelte";
  let create = false;



  $: slug = $page.params.slug;


  $: console.log("Current slug:", slug);


  $: selectedNote = $PatientNotesStore.find(
    (note) => note.id.id.String === $page.params.slug
  );

  $: console.log("selectedNote", selectedNote);
</script>

<h1 class="my-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Notiz Patient {selectedNote?.patient.name}
</h1>


<NoteForm {create} {selectedNote} />