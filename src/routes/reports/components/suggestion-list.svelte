<script lang="ts">
  import type { Suggestion } from "$lib/types/report.types";

  export let suggestions: Suggestion[] = [];
  export let onSuggestionClick: (id: number) => void;

  import { flip } from "svelte/animate";
  import { fade, slide } from "svelte/transition";
  import { quintOut } from 'svelte/easing';
</script>

<div class="h-full flex flex-col" style="height: 60vh;">
  <div class="flex justify-between">
    <h2 class="text-lg font-semibold p-4 pb-2 bg-white">
      Vorschläge
    </h2>
    <p class="p-4" in:fade={{ duration: 600 }}>{suggestions.length}</p>
  </div>
  <div
    class="overflow-y-auto flex-1"
    style="height: calc(60vh - 4rem);"
  >
    <div class="px-4">
      <section class="space-y-2">
        {#each suggestions as suggestion (suggestion.id)}
               <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <div
            on:click={() => onSuggestionClick(suggestion.id)}
            animate:flip={{ duration: 800 }}
            in:slide={{ duration: 300, easing: quintOut }}
            out:slide={{ duration: 500, easing: quintOut }}
            class="cursor-pointer"
          >
            <div
              class="bg-gray-100 p-3 rounded-md hover:bg-gray-200 transition-colors"
              class:border-l-4={suggestion.text.includes("Indikation:")}
              class:border-blue-500={suggestion.text.includes("Indikation:")}
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
