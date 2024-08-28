<script lang="ts">
  import ImageSelect from "../components/image-select.svelte";
  import TextProposals from "../components/text-proposals.svelte";
  import { onMount } from "svelte";
  import * as Resizable from "$lib/components/ui/resizable/index.js";

  let dropzone: HTMLButtonElement;
  let file: File | null = null;
  let reportText: string = "";
  let suggestions: string[] = [
    "Das Bild zeigt...",
    "Bemerkenswerte Merkmale sind...",
    "Die Gesamtkomposition ist...",
    "Die Farbpalette besteht aus...",
    "Im Vordergrund sehen wir...",
    "Der Hintergrund enthält...",
  ];

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const droppedFile = e.dataTransfer?.files[0];
    if (droppedFile) {
      file = droppedFile;
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleClick() {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = "image/*";
    input.onchange = (e: Event) => {
      const target = e.target as HTMLInputElement;
      if (target.files) {
        file = target.files[0];
      }
    };
    input.click();
  }

  function handleDragStart(e: DragEvent, suggestion: string) {
    e.dataTransfer?.setData("text/plain", suggestion);
  }

  function handleTextAreaDrop(e: DragEvent) {
    e.preventDefault();
    const suggestion = e.dataTransfer?.getData("text/plain");
    if (suggestion) {
      reportText += " " + suggestion;
    }
  }

  onMount(() => {
    dropzone.addEventListener("drop", handleDrop);
    dropzone.addEventListener("dragover", handleDragOver);
    return () => {
      dropzone.removeEventListener("drop", handleDrop);
      dropzone.removeEventListener("dragover", handleDragOver);
    };
  });

  $: fileName = file ? file.name : "No file selected";
</script>

<h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Erstelle einen Befund
</h1>

<Resizable.PaneGroup
  direction="horizontal"
  class="max-w min-h-96 rounded-lg border"
>
  <Resizable.Pane defaultSize={150}>
    <Resizable.PaneGroup direction="horizontal">
      <Resizable.Pane defaultSize={50}>
        <ImageSelect />
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
        <TextProposals />
      </Resizable.Pane>
    </Resizable.PaneGroup>
  </Resizable.Pane>
</Resizable.PaneGroup>
