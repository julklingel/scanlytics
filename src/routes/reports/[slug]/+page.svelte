<script lang="ts">
  import { onMount } from "svelte";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Carousel from "$lib/components/ui/carousel/index.js";
  import XIcon from "lucide-svelte/icons/x";
  import { invoke } from "@tauri-apps/api/core";
  import { ReportStore } from "../../../stores/Report";
  import { page } from "$app/stores";
  import { Calendar, User, PersonStanding } from "lucide-svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { toast } from "svelte-sonner";

  export let report_id: string;

  let images: {
    url: string | null | undefined;
    id: string;
    path: string;
    name: string;
  }[] = [];
  let loadedImages: { [key: string]: string } = {};

  $: selectedReport = $ReportStore.find(
    (report) => report.id.id.String === $page.params.slug
  );

  onMount(async () => {
    try {
      const rawImages: any = await invoke("get_report_images", {
        reportId: report_id,
      });
      images = rawImages.map((img: { path: string }) => ({
        ...img,
        url: convertFileSrc(img.path),
      }));
    } catch (error) {
      toast.error("Failed to load images");
    }
  });

  const InfoItem = ({
    icon,
    label,
    value,
  }: {
    icon: any;
    label: any;
    value: any;
  }) => ({
    icon,
    label,
    value,
  });

  let carouselApi: any;

  $: report_text = selectedReport?.reportText || "";
  $: createdAt = formatDate(selectedReport?.createdAt || "");
  $: report_id = selectedReport?.id.id.String;
  $: body_part = selectedReport?.bodyPart;

  function goToSlide(index: number) {
    if (carouselApi) {
      carouselApi.scrollTo(index);
    }
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }
</script>

<h1 class="my-4 mb-8 text-4xl font-extrabold tracking-tight lg:text-5xl">
  Befund von {selectedReport?.patient.name}:
</h1>

<section class="flex gap-4 mb-4 justify-end">
  <Button>Edit</Button>
  <Button variant="destructive">Delete</Button>
</section>

<section class="py-6 bg-white rounded-lg shadow-md p-6">
  <section class="flex flex-col pb-6">
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      {#each [InfoItem( { icon: User, label: "Doctor", value: selectedReport?.userOwner.name } ), InfoItem( { icon: User, label: "Patient", value: selectedReport?.patient.name } ), InfoItem( { icon: PersonStanding, label: "Bodypart", value: selectedReport?.bodyPart } ), InfoItem( { icon: Calendar, label: "Date", value: createdAt } )] as item, i}
        <div class="flex flex-col items-start space-y-1">
          <div class="flex items-center space-x-2">
            <svelte:component
              this={item.icon}
              class="w-5 h-5 text-{['blue', 'green', 'red', 'purple'][i]}-500"
            />
            <h3 class="text-sm font-medium text-gray-500">{item.label}</h3>
          </div>
          <p class="text-lg font-semibold text-gray-900">
            {item.value || "N/A"}
          </p>
        </div>
      {/each}
    </div>
  </section>

  <Resizable.PaneGroup
    direction="horizontal"
    class="max-w-full h-[80vh] rounded-lg border"
  >
    <Resizable.Pane defaultSize={150}>
      <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={50}>
          {#if images.length > 0}
            <div class="relative h-full">
              <Carousel.Root bind:api={carouselApi}>
                <Carousel.Content>
                  {#each images as image, index (image.id.id.String)}
                    <Carousel.Item>
                      <div class="relative w-full h-full max-h-[60vh] flex items-center justify-center">
                        <img
                          src={image.url}
                          alt={`Uploaded image ${index + 1}`}
                          class="object-contain max-w-full max-h-full"
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
                {#each images as image, index (image.id.id.String)}
                  <button
                    class="w-6 h-6 bg-white border border-gray-300 rounded-full"
                    on:click={() => goToSlide(index)}
                  >
                    <img
                      src={image.url}
                      alt={`Thumbnail ${index + 1}`}
                      class="w-full h-full object-cover rounded-full"
                    />
                  </button>
                {/each}
              </div>
            </div>
          {:else}
            <section class="flex justify-center items-center h-full">
              Place holder image
            </section>
          {/if}
        </Resizable.Pane>

        <Resizable.Handle />

        <Resizable.Pane defaultSize={50}>
          <div class="flex flex-col h-full p-4">
            <textarea
              bind:value={report_text}
              class="flex-grow p-2 border rounded-md resize-none overflow-y-auto"
              placeholder="Schreiben Sie Ihren Befund hier..."
              disabled
            />
          </div>
        </Resizable.Pane>

        <Resizable.Handle />
      </Resizable.PaneGroup>
    </Resizable.Pane>
  </Resizable.PaneGroup>
</section>
