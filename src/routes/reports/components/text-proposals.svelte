<script lang="ts">
    import { Label } from "$lib/components/ui/label/index.js";
    import { onMount } from "svelte";
    import * as Resizable from "$lib/components/ui/resizable/index.js";
  
    let dropzone: HTMLButtonElement;
    let file: File | null = null;
    let reportText: string = '';
    let suggestions: string[] = [
      'Das Bild zeigt...',
      'Bemerkenswerte Merkmale sind...',
      'Die Gesamtkomposition ist...',
      'Die Farbpalette besteht aus...',
      'Im Vordergrund sehen wir...',
      'Der Hintergrund enthält...',
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
  
  
    function handleDragStart(e: DragEvent, suggestion: string) {
      e.dataTransfer?.setData('text/plain', suggestion);
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

          <div class="h-full p-4 overflow-y-auto">
            <h2 class="text-lg font-semibold mb-2">Vorschläge</h2>
            {#each suggestions as suggestion}
              <!-- svelte-ignore a11y-no-static-element-interactions -->
              <div
                class="bg-gray-100 p-2 mb-2 rounded-md cursor-grab"
                draggable="true"
                on:dragstart={(e) => handleDragStart(e, suggestion)}
              >
                {suggestion}
              </div>
            {/each}
          </div>

