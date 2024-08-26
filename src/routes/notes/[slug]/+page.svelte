<script lang="ts">
  import { PatientNotesStore } from "../../../stores/PatientNote";
  import { page } from "$app/stores";
  import { Calendar, User, Thermometer } from 'lucide-svelte';
  import Button from "$lib/components/ui/button/button.svelte";

  let create = false;

  $: selectedNote = $PatientNotesStore.find(
    (note) => note.id.id.String === $page.params.slug
  );

  const InfoItem = ({ icon, label, value }) => ({
    icon,
    label,
    value
  });

  $: detailSections = [
    { title: 'Symptoms', content: selectedNote?.symptoms, bgColor: 'bg-yellow-50' },
    { title: 'Diagnosis', content: selectedNote?.diagnosis, bgColor: 'bg-blue-50' },
    { title: 'Treatment', content: selectedNote?.treatment, bgColor: 'bg-green-50' }
  ];

  $: console.log("selectedNote", selectedNote);
  $: console.log("detailSections", detailSections);
</script>

<h1 class="my-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Notiz Patient {selectedNote?.patient.name}
</h1>

<section class="flex gap-4 mb-4 justify-end">
  <Button>Edit</Button>
  <Button variant="destructive">Delete</Button>

</section>

<section class="py-6 bg-white rounded-lg shadow-md p-6">
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
    {#each [
      InfoItem({ icon: User, label: 'Doctor', value: selectedNote?.userOwner.name }),
      InfoItem({ icon: User, label: 'Patient', value: selectedNote?.patient.name }),
      InfoItem({ icon: Thermometer, label: 'Severity', value: selectedNote?.severity }),
      InfoItem({ icon: Calendar, label: 'Date', value: selectedNote?.createdAt })
    ] as item, i}
      <div class="flex flex-col items-start space-y-1">
        <div class="flex items-center space-x-2">
          <svelte:component this={item.icon} class="w-5 h-5 text-{['blue', 'green', 'red', 'purple'][i]}-500" />
          <h3 class="text-sm font-medium text-gray-500">{item.label}</h3>
        </div>
        <p class="text-lg font-semibold text-gray-900">{item.value || 'N/A'}</p>
      </div>
    {/each}
  </div>
  
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    {#each detailSections as { title, content, bgColor }}
      <div class="{bgColor} rounded-lg p-4 shadow">
        <h2 class="text-lg font-bold mb-2">{title}</h2>
        <p class="text-gray-700">{content || 'No information available'}</p>
      </div>
    {/each}
  </div>
</section>