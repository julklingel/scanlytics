<script lang="ts">
    import TextProposals from '../components/text-proposals.svelte';
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
      e.dataTransfer?.setData('text/plain', suggestion);
    }
  
    function handleTextAreaDrop(e: DragEvent) {
      e.preventDefault();
      const suggestion = e.dataTransfer?.getData('text/plain');
      if (suggestion) {
        reportText += ' ' + suggestion;
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

          <section class="flex justify-center items-center h-full">
            <div class="flex flex-col items-center">
              <button
                bind:this={dropzone}
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
                <div class="mt-4 flex justify-center text-sm leading-6 text-gray-600">
                  <label
                    for="file-upload"
                    class="flex relative cursor-pointer rounded-md bg-white font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500"
                  >
                    <span>Upload a file</span>
                  </label>
                </div>
                <p class="text-xs leading-5 text-gray-600">PNG, JPG, BMP up to 10MB</p>
              </button>
              {#if file}
                <p class="mt-2 text-sm text-gray-500">Selected file: {fileName}</p>
              {/if}
            </div>
          </section>

