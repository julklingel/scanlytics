<script lang="ts">
    import DarkMode from '../../components/ui/darkmode.svelte';
    import { Button } from "$lib/components/ui/button";
    import { goto } from '$app/navigation';


    async function navigate() {
		console.log("Navigating to /menu");
        await goto("/menu");
    }
</script>

<section class="flex flex-col h-screen items-center justify-center">
    <h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">Scanlytics</h1>
    <p class="mt-6 pl-6 italic">#1 Text Editor in der Medizin</p>
    <br>
    <Button on:click={navigate} >Menu </Button>
</section>