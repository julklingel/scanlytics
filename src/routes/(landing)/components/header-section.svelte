<script lang="ts">
	import DarkMode from '../../components/ui/darkmode.svelte';
	import { Button } from '$lib/components/ui/button';

	import { goto } from '$app/navigation';


	async function navigate2menu() {
    await goto("/menu");
  }
</script>

<div class="absolute right-0 m-2">
	<div class="flex">
		<Button variant={"link"} on:click={navigate2menu}>Directly to Menu - DEV</Button>
	
		<DarkMode />
	</div>
</div>