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
  import ImageCarousel from "../components/image-carousel.svelte";
  import ReportTextArea from "../components/report-text-area.svelte";
  import SuggestionList from "../components/suggestion-list.svelte";

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
  $: if (files.length === 0) {
    suggestions = [];
    addedSugg = [];
  }

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

  function processStatementsToSuggestions(
    statements: ModelResponse["statements"]
  ) {
    let nextId = suggestions.length + 1;

    const newSuggestions = statements.map((statement) => ({
      id: nextId++,
      text: `Indikation: ${statement.indication}\n${statement.statement}\nBefund: ${statement.assessment}`,
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
    const suggestionIndex = suggestions.findIndex(
      (s: Suggestion) => s.id === id
    );
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
  <ReportMetaData bind:patient_id bind:user_owner />
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
          <ImageCarousel bind:files bind:carouselApi />
        </Resizable.Pane>

        <Resizable.Handle />

        <Resizable.Pane defaultSize={50}>
          <ReportTextArea bind:report_text onRevert={revertLastSugg} />
        </Resizable.Pane>

        <Resizable.Handle />

        <Resizable.Pane defaultSize={50}>
          <SuggestionList {suggestions} onSuggestionClick={addSugg} />
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

</style>
