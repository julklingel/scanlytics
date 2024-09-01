<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import { onMount } from "svelte";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { flip } from "svelte/animate";
  import PatientCombobox from "../components/patient-combobox.svelte";
  import DoctorCombobox from "../components/doctor-combobox.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Carousel from "$lib/components/ui/carousel/index.js";
  import PlusIcon from "lucide-svelte/icons/plus";
  import XIcon from "lucide-svelte/icons/x";
  import { toast } from "svelte-sonner";
  import { invoke } from "@tauri-apps/api/core";
  import { getPatients } from "../api/patient-data";
  import { getUsers } from "../api/user-data";


  export let patient_id: string;
  export let userOwner: string;

  let files: File[] = [];
  let reportText: string = "";

  onMount(async () => {
    try {
      await getUsers();
      await getPatients();
    } catch (error) {
      console.error(error);
    }
  });


  let carouselApi: any;

  export let suggestions: { id: number; text: string }[] = [
    {
      id: 1,
      text: "Das Bild zeigt...kugjlhgjhfhgkfkhgf ghc  zvckckhgf  wbdfowafdweiubdwequ weubdqwzbd wuebdwebffbre piwqudbewqiufbd biuefbiewquf iaefbpiwerbfu aieuzbfabf",
    },
    { id: 2, text: "Bemerkenswerte Merkmale sind..." },
    { id: 3, text: "Die Gesamtkomposition ist..." },
    { id: 4, text: "Die Farbpalette besteht aus..." },
    { id: 5, text: "Im Vordergrund sehen wir..." },
    { id: 6, text: "Der Hintergrund enthält..." },
  ];

  let addedSugg: { id: number; text: string }[] = [];

  async function handleSubmit() {
    const formData = {
      patient_id,
      userOwner,
      reportText,
      files,
    };
    try {
      await invoke("create_report", formData);
      toast.success("Report created successfully");
    } catch (error) {
      console.error(error);
      toast.error("Failed to create report");
    }
  }

  function handleClick() {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = "image/*";
    input.multiple = true;
    input.onchange = (e: Event) => {
      const target = e.target as HTMLInputElement;
      if (target.files) {
        files = [...files, ...Array.from(target.files)];
      }
    };
    input.click();
  }

  function goToSlide(index: number) {
    if (carouselApi) {
      carouselApi.scrollTo(index);
    }
  }
  function removeImage(index: number) {
    files = files.filter((_, i) => i !== index);
    if (files.length > 0 && carouselApi) {
      carouselApi.scrollTo(Math.min(index, files.length - 1));
    }
  }

  function addSugg(id: number) {
    const suggestionIndex = suggestions.findIndex((s) => s.id === id);
    if (suggestionIndex !== -1) {
      const suggestion = suggestions[suggestionIndex];
      reportText += suggestion.text;
      suggestions = suggestions.filter((s) => s.id !== id);
      addedSugg = [...addedSugg, suggestion];
    }
  }

  function revertLastSugg() {
    if (addedSugg.length > 0) {
      const lastSuggestion = addedSugg.pop();
      if (lastSuggestion) {
        reportText = reportText.slice(0, -lastSuggestion.text.length);
        suggestions = [...suggestions, lastSuggestion];
        addedSugg = [...addedSugg];
      }
    }
  }

  $: fileNames = files.map((file) => file.name).join(", ");
</script>

<h1 class="my-4 mb-8 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Erstelle einen Befund:
</h1>

<section class="flex flex-col pb-6">
  <div class="flex gap-60">
    <div class="">
      <Label for="patientName" class="block text-sm font-medium text-gray-700"
        >Patient</Label
      >
      <PatientCombobox bind:patient_id />
    </div>
    <div class="">
      <Label
        for="attendingDoctor"
        class="block text-sm font-medium text-gray-700">Attending Doctor</Label
      >
      <DoctorCombobox bind:selectedDoctorId={userOwner} />
    </div>
    <div class="">
      <Label
        for="attendingDoctor"
        class="block text-sm font-medium text-gray-700">Körperteil</Label>
        <Input type="text" placeholder="Search for type" class="mt-1 block w-full" />
    </div>

  </div>
</section>

<Resizable.PaneGroup
  direction="horizontal"
  class="max-w min-h-96 rounded-lg border"
>
<Resizable.Pane defaultSize={150}>
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane defaultSize={50}>
      {#if files.length > 0}
        <div class="relative">
          <Carousel.Root bind:api={carouselApi}>
            <Carousel.Content>
              {#each files as file, index (file)}
                <Carousel.Item>
                  <div class="relative">
                    <button
                      on:click={() => removeImage(index)}
                      class="absolute top-2 right-2 bg-white rounded-full p-1 shadow-md"
                    >
                      <XIcon size={16} />
                    </button>
                    <img
                      src={URL.createObjectURL(file)}
                      alt={`Uploaded image ${index + 1}`}
                      class="object-cover w-full h-full"
                      aria-hidden="true"
                    />
                  </div>
                </Carousel.Item>
              {/each}
            </Carousel.Content>
            <Carousel.Previous />
            <Carousel.Next />
          </Carousel.Root>
            <div
              class="absolute bottom-4 left-0 right-0 flex justify-center space-x-2"
            >
              {#each files as _, index (index)}
                <button
                  class="w-8 h-8 bg-white border border-gray-300 rounded-full"
                  on:click={() => goToSlide(index)}
                >
                  <img
                    src={URL.createObjectURL(files[index])}
                    alt={`Thumbnail ${index + 1}`}
                    class="w-full h-full object-cover rounded-full"
                  />
                </button>
              {/each}

              <button
                class=" flex justify-center items-center bg-gray-300 w-8 h-8 border border-gray-300 rounded-full"
                on:click={handleClick}
              >
                <PlusIcon size={12} />
              </button>
            </div>
          </div>
        {:else}
          <section class="flex justify-center items-center h-full">
            <div class="flex flex-col items-center">
              <button
                on:click={handleClick}
                class="flex flex-col items-center py-6 m-4 bg-white rounded-lg shadow-md p-6 cursor-pointer"
              >
                <svg
                  class="h-12 w-12 text-gray-300"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path
                    fill-rule="evenodd"
                    d="M1.5 6a2.25 2.25 0 012.25-2.25h16.5A2.25 2.25 0 0122.5 6v12a2.25 2.25 0 01-2.25 2.25H3.75A2.25 2.25 0 011.5 18V6zM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0021 18v-1.94l-2.69-2.689a1.5 1.5 0 00-2.12 0l-.88.879.97.97a.75.75 0 11-1.06 1.06l-5.16-5.159a1.5 1.5 0 00-2.12 0L3 16.061zm10.125-7.81a1.125 1.125 0 112.25 0 1.125 1.125 0 01-2.25 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <div
                  class="mt-4 flex justify-center text-sm leading-6 text-gray-600"
                >
                  <label
                    for="file-upload"
                    class="flex relative cursor-pointer rounded-md bg-white font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500"
                  >
                    <span>Upload files</span>
                  </label>
                </div>
                <p class="text-xs leading-5 text-gray-600">
                  PNG, JPG, BMP up to 10MB each
                </p>
              </button>
            </div>
          </section>
        {/if}
      </Resizable.Pane>

      <Resizable.Handle />

      <Resizable.Pane defaultSize={50}>
        <div class="flex flex-col h-full p-4">
          <div class="flex justify-between items-center py-2">
            <h2 class="text-lg font-semibold">Befund</h2>
            <Button variant="secondary" on:click={revertLastSugg}>Revert</Button
            >
          </div>
          <textarea
            bind:value={reportText}
            class="flex-grow p-2 border rounded-md resize-none"
            placeholder="Schreiben Sie Ihren Befund hier..."
          />
        </div>
      </Resizable.Pane>

      <Resizable.Handle />

      <Resizable.Pane defaultSize={50}>
        <div class="h-full p-4 overflow-y-auto">
          <h2 class="text-lg font-semibold mb-2">Vorschläge</h2>
          <section>
            {#each suggestions as suggestion (suggestion.id)}
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <!-- svelte-ignore a11y-no-static-element-interactions -->
              <div
                on:click={() => addSugg(suggestion.id)}
                animate:flip={{ duration: 200 }}
              >
                <div class="bg-gray-100 p-2 mb-2 rounded-md">
                  {suggestion.text}
                </div>
              </div>
            {/each}
          </section>
        </div>
      </Resizable.Pane>
    </Resizable.PaneGroup>
  </Resizable.Pane>
</Resizable.PaneGroup>

<section class="flex justify-end my-6 gap-4">
  <!-- TBD -->
  <!-- <Button class=" ">Preview</Button> -->
  <Button on:click={handleSubmit} class=" ">Save</Button>
</section>
