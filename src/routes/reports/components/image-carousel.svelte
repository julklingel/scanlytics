<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import PlusIcon from "lucide-svelte/icons/plus";
    import XIcon from "lucide-svelte/icons/x";
    import type { CarouselApi } from "$lib/types/report.types";
  
    export let files: File[] = [];
    export let carouselApi: CarouselApi;
  
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
  </script>
  
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
      <div class="absolute bottom-4 left-0 right-0 flex justify-center space-x-2">
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
          class="flex justify-center items-center bg-gray-300 w-8 h-8 border border-gray-300 rounded-full"
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
          <div class="mt-4 flex justify-center text-sm leading-6 text-gray-600">
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
  