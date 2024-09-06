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
  import { ReportStore } from "../../../stores/Report";
  import { page } from "$app/stores";
  import { Calendar, User, PersonStanding } from 'lucide-svelte';


  export let patient_id: string;
  export let user_owner: string;

  $: selectedReport = $ReportStore.find(
  (report) => report.id.id.String === $page.params.slug
);

const InfoItem = ({ icon, label, value }: { icon: any, label: any, value: any }) => ({
    icon,
    label,
    value
  });

  

  let carouselApi: any;
  let files: File[] = [];
  $: report_text = selectedReport?.reportText || "";
  $: createdAt = formatDate(selectedReport?.createdAt || "") 

  $: console.log(selectedReport?.reportText);


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

  function goToSlide(index: number) {
    if (carouselApi) {
      carouselApi.scrollTo(index);
    }
  }
  function removeImage(index: number) {
    files = files.filter((_, i) => i !== index);
    if (files.length > 0 && carouselApi) {
      carouselApi.scrollTo(Math.min(index, files.length - 1));
    }
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }

  $: fileNames = files.map((file) => file.name).join(", ");
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
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
    {#each [
      InfoItem({ icon: User, label: 'Doctor', value: selectedReport?.userOwner.name }),
      InfoItem({ icon: User, label: 'Patient', value: selectedReport?.patient.name }),
      InfoItem({ icon : PersonStanding, label: 'Bodypart', value: selectedReport?.bodyType }),
      InfoItem({ icon: Calendar, label: 'Date', value: createdAt })
     
    ] as item, i}
      <div class="flex flex-col items-start space-y-1">
        <div class="flex items-center space-x-2">
          <svelte:component this={item.icon} class="w-5 h-5 text-{['blue', 'green', 'red', 'purple'][i]}-500" />
          <h3 class="text-sm font-medium text-gray-500">{item.label}</h3>
        </div>
        <p class="text-lg font-semibold text-gray-900">{item.value || 'N/A'}</p>
      </div>
    {/each}
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
                {#each files as file, index (file)}
                  <Carousel.Item>
                    <div class="relative">
                      <button
                        on:click={() => removeImage(index)}
                        class="absolute top-2 right-2 bg-white rounded-full p-1 shadow-md"
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
            class="flex-grow p-2 border rounded-md resize-none"
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