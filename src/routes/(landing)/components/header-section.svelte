<script lang="ts">
	import DarkMode from '../../components/ui/darkmode.svelte';
	import Button from '../../components/ui/button.svelte';
	export let buttonText = 'Sign Up';
    export let variant: "link" | "default" | "destructive" | "outline" | "secondary" | "ghost" = "link";
</script>

<div class="absolute right-0 m-2">
	<div class="flex">
		<Button {buttonText} {variant} />
		<DarkMode />
	</div>
</div>