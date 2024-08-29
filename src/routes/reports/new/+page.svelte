<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import { onMount } from "svelte";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { flip } from "svelte/animate";
  import { createEventDispatcher } from "svelte";
  import PatientCombobox from "../components/patient-combobox.svelte";
  import DoctorCombobox from "../components/doctor-combobox.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Carousel from "$lib/components/ui/carousel/index.js";

  export let patient_id: string;
  export let userOwner: string;

  let files: File[] = [];
  let reportText: string = "";
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

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    console.log("handleDragOver");
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

  function handleDragStart(
    e: DragEvent,
    suggestion: { id: number; text: string }
  ) {
    console.log("handleDragStart");
    console.log("At start -->", suggestion);
    e.dataTransfer?.setData("text/plain", suggestion.text);
  }

  function handleTextAreaDrop(e: DragEvent) {
    console.log("handleTextAreaDrop");
    const suggestion = e.dataTransfer?.getData("text/plain");
    if (suggestion) {
      reportText += " " + suggestion;
    }
  }

  function goToSlide(index: number) {
    if (carouselApi) {
      carouselApi.scrollTo(index);
    }
  }

  $: fileNames = files.map(file => file.name).join(", ");
</script>

<h1 class="my-4 mb-8 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Erstelle einen Befund
</h1>

<section class="flex flex-col pb-6">
  <div class="flex justify-between">
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
        class="block text-sm font-medium text-gray-700">Körperteil</Label
      >
    </div>
    <div class="">
      <Label
        for="attendingDoctor"
        class="block text-sm font-medium text-gray-700">Kondition</Label
      >
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
                  {#each files as file, index (index)}
                    <Carousel.Item>
                      <img
                        src={URL.createObjectURL(file)}
                        alt={`Uploaded image ${index + 1}`}
                        class="object-cover w-full h-full"
                        aria-hidden="true"
                      />
                    </Carousel.Item>
                  {/each}
                </Carousel.Content>
                <Carousel.Previous />
                <Carousel.Next />
              </Carousel.Root>
              <div class="absolute bottom-4 left-0 right-0 flex justify-center space-x-2">
                {#each files as _, index (index)}
                  <button
                    class="w-8 h-8 bg-white border border-gray-300 rounded-full "
                    on:click={() => goToSlide(index)}
                  >
                    <img
                      src={URL.createObjectURL(files[index])}
                      alt={`Thumbnail ${index + 1}`}
                      class="w-full h-full object-cover rounded-full"
                    />
                  </button>
                {/each}
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
                {#if files.length > 0}
                  <p class="mt-2 text-sm text-gray-500">
                    Selected files: {fileNames}
                  </p>
                {/if}
              </div>
            </section>
          {/if}
      </Resizable.Pane>

      <Resizable.Handle />

      <Resizable.Pane defaultSize={50}>
        <div class="flex flex-col h-full p-4">
          <h2 class="text-lg font-semibold mb-2">Befund</h2>
          <textarea
            class="flex-grow p-2 border rounded-md resize-none"
            bind:value={reportText}
            on:drop={handleTextAreaDrop}
            on:dragover={handleDragOver}
            placeholder="Schreiben Sie Ihren Befund hier..."
          />
        </div>
      </Resizable.Pane>

      <Resizable.Handle />

      <Resizable.Pane defaultSize={50}>
        <div class="h-full p-4 overflow-y-auto">
          <h2 class="text-lg font-semibold mb-2">Vorschläge</h2>
          <ul>
            {#if suggestions.length === 0}
              <li class="text-gray-500">Keine Vorschläge vorhanden</li>
            {:else}
              {#each suggestions as suggestion, index (suggestion.id)}
                <li
                  class="bg-gray-100 p-2 mb-2 rounded-md cursor-grab"
                  draggable="true"
                  on:dragstart={(e) => handleDragStart(e, suggestion)}
                  on:dragover={handleDragOver}
                  on:drop|preventDefault={handleTextAreaDrop}
                  animate:flip={{ duration: 300 }}
                >
                  {suggestion.text}
                </li>
              {/each}
            {/if}
          </ul>
        </div>
      </Resizable.Pane>
    </Resizable.PaneGroup>
  </Resizable.Pane>
</Resizable.PaneGroup>

<section class="flex justify-end my-6">
  <Button class=" ">Save</Button>
</section>