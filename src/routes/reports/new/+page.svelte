<script lang="ts">
    import { Label } from "$lib/components/ui/label/index.js";
    import { onMount } from 'svelte';

    let dropzone: HTMLButtonElement;
    let file: File | null = null;

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
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'image/*';
        input.onchange = (e: Event) => {
            const target = e.target as HTMLInputElement;
            if (target.files) {
                file = target.files[0];
            }
        };
        input.click();
    }

    onMount(() => {
        dropzone.addEventListener('drop', handleDrop);
        dropzone.addEventListener('dragover', handleDragOver);
        return () => {
            dropzone.removeEventListener('drop', handleDrop);
            dropzone.removeEventListener('dragover', handleDragOver);
        };
    });

    $: fileName = file ? file.name : 'No file selected';
</script>

<h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">
    Erstelle einen Befund
</h1>

<section class="flex justify-between gap-4">
    <div class="w-1/3">
        <Label for="picture">Picture</Label>
        <button
            bind:this={dropzone}
            on:click={handleClick}
            class="mt-2 flex justify-center rounded-lg border border-dashed border-gray-900/25 px-6 py-10 cursor-pointer"
        >
            <div class="text-center">
                <svg class="mx-auto h-12 w-12 text-gray-300" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M1.5 6a2.25 2.25 0 012.25-2.25h16.5A2.25 2.25 0 0122.5 6v12a2.25 2.25 0 01-2.25 2.25H3.75A2.25 2.25 0 011.5 18V6zM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0021 18v-1.94l-2.69-2.689a1.5 1.5 0 00-2.12 0l-.88.879.97.97a.75.75 0 11-1.06 1.06l-5.16-5.159a1.5 1.5 0 00-2.12 0L3 16.061zm10.125-7.81a1.125 1.125 0 112.25 0 1.125 1.125 0 01-2.25 0z" clip-rule="evenodd" />
                </svg>
                <div class="mt-4 flex text-sm leading-6 text-gray-600">
                    <label
                        for="file-upload"
                        class="relative cursor-pointer rounded-md bg-white font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500"
                    >
                        <span>Upload a file</span>
                    </label>
                    <p class="pl-1">or drag and drop</p>
                </div>
                <p class="text-xs leading-5 text-gray-600">PNG, JPG, GIF up to 10MB</p>
            </div>
        </button>
        {#if file}
            <p class="mt-2 text-sm text-gray-500">Selected file: {fileName}</p>
        {/if}
    </div>
</section>

