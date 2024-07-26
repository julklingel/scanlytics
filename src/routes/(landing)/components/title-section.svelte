<script lang="ts">
	import DarkMode from '../../../components/ui/darkmode.svelte';
	import Button from '../../../components/ui/button.svelte';
	export let buttonText = 'Login';
	export let link = './menu';
    export let variant: "link" | "default" | "destructive" | "outline" | "secondary" | "ghost" = "default";
</script>

<section class="flex flex-col h-screen items-center justify-center">
	<h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">Scanlytics</h1>
	<p class="mt-6 pl-6 italic">#1 Text Editor in der Medizin</p>
    <br>
    <Button {link} {buttonText} {variant}/>
</section>