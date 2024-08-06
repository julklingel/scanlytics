<script lang="ts">
	import DarkMode from '../../components/ui/darkmode.svelte';
	import { Button } from "$lib/components/ui/button";

    export let variant: "link" | "default" | "destructive" | "outline" | "secondary" | "ghost" = "link";
</script>

<div class="absolute right-0 m-2">
	<div class="flex">
		<Button {variant}> Signup </Button>
		<DarkMode />
	</div>
</div>