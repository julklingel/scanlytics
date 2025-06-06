<script lang="ts">
  import { onMount } from "svelte";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
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
  import { processImages } from "../api/process-images";

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
    setTimeout(() => {
      suggestions = [];
      addedSugg = [];
    }, 300);
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
      label: "Accurate",
      type: "body_part_classifier_resnet18",
      variant: "secondary",
      selected: false,
    },
    // Still needs to be discussed wether a third model is needed
    // {
    //   id: 3,
    //   label: "Accurate",
    //   type: "body_part_classifier_resnet18",
    //   variant: "secondary",
    //   selected: false,
    // },
  ];

  let selectedModel: string;
  $: selectedModel = models.find((model) => model.selected)?.type || "body_part_classifier";

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

  $: {
    if (files.length > 0 && selectedModel) {
      processImages(files, active_user, selectedModel).then(
        (response: ModelResponse | null) => {
          if (response) {
            processStatementsToSuggestions(response.statements);
          }
        }
      );
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
