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
  import { goto } from "$app/navigation";
  import AuthService from "../../../stores/Auth";
  import ReportMetaData from "../components/report-meta-data.svelte";

  import type {
    Model,
    ModelResponse,
    Suggestion,
    FileData,
    ReportData,
    CarouselApi,
  } from "$lib/types/report.types";

  export let active_user: string;
  $: active_user = $AuthService.user_email;

  let user_owner: string;

  export let patient_id: string;
  let body_part: string = "";

  let models: Model[] = [
    {
      id: 1,
      label: "Speedy",
      type: "body_part_classifier",
      variant: "secondary",
      selected: true,
    },
    {
      id: 2,
      label: "Balanced",
      type: "ResNet16",
      variant: "secondary",
      selected: false,
    },
    {
      id: 3,
      label: "Accurate",
      type: "ResNet60",
      variant: "secondary",
      selected: false,
    },
  ];

  let selectedModel: string;
  $: selectedModel = models.find((model) => model.selected)?.type || "MNST_med";

  let carouselApi: CarouselApi;
  let files: File[] = [];
  let report_text: string = "";

  let suggestions: Suggestion[] = [];
  let addedSugg: Suggestion[] = [];

  function processStatementsToSuggestions(statements: ModelResponse['statements']) {
    let nextId = suggestions.length + 1;
    
    const newSuggestions = statements.map(statement => ({
      id: nextId++,
      text: `Indikation: ${statement.indication}\n${statement.statement}\nBefund: ${statement.assessment}`
    }));

    suggestions = [...newSuggestions];
  }

  function selectModel(selectedModel: Model) {
    models = models.map((model) => ({
      ...model,
      variant: model.id === selectedModel.id ? "default" : "secondary",
      selected: model.id === selectedModel.id ? true : false,
    }));
  }

  async function sendFilesToBackend(files: File[]) {
    try {
      let fileData: FileData[] = await Promise.all(
        files.map(async (file) => ({
          filename: file.name,
          extension: file.name.split(".").pop() || "",
          data: Array.from(new Uint8Array(await file.arrayBuffer())),
        }))
      );

      const result: ModelResponse = await invoke("process_images", {
        imageData: JSON.stringify(fileData),
        userName: JSON.stringify(active_user),
        modelName: JSON.stringify(selectedModel),
      });

      toast.success("Images processed successfully");
      console.log("Images processed:", result);

      if (result.statements && result.statements.length > 0) {
        processStatementsToSuggestions(result.statements);
      }

      result.results.forEach((imageResult) => {
        console.log(
          `File: ${imageResult.filename}, Type: ${imageResult.image_type}, Confidence: ${imageResult.confidence}`
        );
   
        if (!body_part && imageResult.confidence > 0.5) {
          body_part = imageResult.image_type;
        }
      });
    } catch (error) {
      console.error("Error processing images:", error);
      toast.error("Error processing images");
    }
  }

  $: {
  if (files.length > 0 && selectedModel) {
    sendFilesToBackend(files);
  }
}

onMount(async () => {
  try {
    await getUsers();
    await getPatients();
  } catch (error) {
    console.error(error);
  }
});

async function fileToUint8Array(file: File): Promise<Uint8Array> {
  return new Uint8Array(await file.arrayBuffer());
}

async function handleSubmit() {
  const fileData: FileData[] = await Promise.all(
    files.map(async (file) => ({
      filename: file.name,
      extension: file.name.split(".").pop() || "",
      data: Array.from(await fileToUint8Array(file)),
    }))
  );

  const reportData: ReportData = {
    patient_id,
    user_owner,
    body_part,
    report_text,
    files: fileData,
  };

  try {
    const response = await invoke("create_report", {
      reportRequest: JSON.stringify(reportData),
    });
    toast.success("Report created successfully");
    goto("/reports");
  } catch (error) {
    console.error(error);
    let stringerror = JSON.stringify(error);
    toast.error(stringerror);
  }
}

function handleClick(): void {
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

function goToSlide(index: number): void {
  if (carouselApi) {
    carouselApi.scrollTo(index);
  }
}

function removeImage(index: number): void {
  files = files.filter((_, i) => i !== index);
  if (files.length > 0 && carouselApi) {
    carouselApi.scrollTo(Math.min(index, files.length - 1));
  }
}

function addSugg(id: number): void {
  const suggestionIndex = suggestions.findIndex((s: Suggestion) => s.id === id);
  if (suggestionIndex !== -1) {
    const suggestion: Suggestion = suggestions[suggestionIndex];
    report_text += suggestion.text;
    suggestions = suggestions.filter((s: Suggestion) => s.id !== id);
    addedSugg = [...addedSugg, suggestion];
  }
}

function revertLastSugg(): void {
  if (addedSugg.length > 0) {
    const lastSuggestion: Suggestion | undefined = addedSugg.pop();
    if (lastSuggestion) {
      report_text = report_text.slice(0, -lastSuggestion.text.length);
      suggestions = [lastSuggestion, ...suggestions];
      addedSugg = [...addedSugg];
    }
  }
}
</script>

<h1 class="my-4 mb-8 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Erstelle einen Befund:
</h1>

<section class="flex flex-col pb-6">
  <ReportMetaData 
    bind:patient_id
    bind:user_owner
  />
</section>


<section>
  <div class="flex gap-2 my-2">
    {#each models as model (model.id)}
      <Button
        on:click={() => selectModel(model)}
        variant={model.selected ? "default" : "secondary"}
      >
        {model.label}
      </Button>
    {/each}
  </div>
</section>

<section>
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
                          class="absolute z-10 top-2 right-2 bg-white rounded-full p-1 shadow-md"
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
              <Button variant="secondary" on:click={revertLastSugg}
                >Revert</Button
              >
            </div>
            <textarea
              bind:value={report_text}
              class="flex-grow p-2 border rounded-md resize-none"
              placeholder="Schreiben Sie Ihren Befund hier..."
            />
          </div>
        </Resizable.Pane>

        <Resizable.Handle />

        <Resizable.Pane defaultSize={50}>
          <div class="h-full flex flex-col" style="height: 60vh;">
            <!-- Fixed full viewport height -->
            <div class="flex justify-between">
              <h2 class="text-lg font-semibold p-4 pb-2 bg-white">
                Vorschläge
              </h2>
              <p class="p-4">{suggestions.length}</p>
            </div>
            <div
              class="overflow-y-auto flex-1"
              style="height: calc(60vh - 4rem);"
            >
              <!-- Subtract header height -->
              <div class="px-4">
                <section class="space-y-2">
                  {#each suggestions as suggestion (suggestion.id)}
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <!-- svelte-ignore a11y-no-static-element-interactions -->
                    <div
                      on:click={() => addSugg(suggestion.id)}
                      animate:flip={{ duration: 200 }}
                      class="cursor-pointer"
                    >
                      <div
                        class="bg-gray-100 p-3 rounded-md hover:bg-gray-200 transition-colors"
                        class:border-l-4={suggestion.text.includes(
                          "Indikation:"
                        )}
                        class:border-blue-500={suggestion.text.includes(
                          "Indikation:"
                        )}
                      >
                        {#if suggestion.text.includes("Indikation:")}
                          <div class="text-sm space-y-1">
                            {#each suggestion.text.split("\n") as line}
                              <p
                                class={line.startsWith("Befund:")
                                  ? "font-semibold mt-1"
                                  : line.startsWith("Indikation:")
                                    ? "text-blue-600"
                                    : "text-gray-700"}
                              >
                                {line}
                              </p>
                            {/each}
                          </div>
                        {:else}
                          <p class="text-gray-700">{suggestion.text}</p>
                        {/if}
                      </div>
                    </div>
                  {/each}
                </section>
              </div>
            </div>
          </div>
        </Resizable.Pane>
      </Resizable.PaneGroup>
    </Resizable.Pane>
  </Resizable.PaneGroup>
</section>

<section class="flex justify-end my-6 gap-4">
  <!-- TBD -->
  <!-- <Button class=" ">Preview</Button> -->
  <Button on:click={handleSubmit} class=" ">Save</Button>
</section>

<style>
  :global(.overflow-y-auto) {
    scrollbar-width: thin;
    scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
  }

  :global(.overflow-y-auto::-webkit-scrollbar) {
    width: 6px;
  }

  :global(.overflow-y-auto::-webkit-scrollbar-track) {
    background: transparent;
  }

  :global(.overflow-y-auto::-webkit-scrollbar-thumb) {
    background-color: rgba(156, 163, 175, 0.5);
    border-radius: 3px;
  }

  :global(.overflow-y-auto::-webkit-scrollbar-thumb:hover) {
    background-color: rgba(156, 163, 175, 0.7);
  }
</style>
