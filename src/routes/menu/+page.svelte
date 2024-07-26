<script lang="ts">
	import Card from '../../components/ui/card.svelte';
	import AppHeader from '../../components/ui/file-header.svelte';
</script>

<AppHeader />

<h1 class="m-4 text-4xl font-extrabold tracking-tight lg:text-5xl">Menu:</h1>

<section class="m-4 mt-8 flex items-center justify-center">
	<div class="flex flex-wrap gap-4">
		<Card title="Patienten" description="Verwalte deine Patienten" link={'./patients'} />
		<Card title="Befunde" description="Erstelle einen Befund" link={'./reports'} />
		<Card title="Notizen" description="Verwalte deine Notizen" link={'./notes'} />
		<Card title="Rechnungen" description="Erstelle eine Rechung" link={'./invoices'} />
	</div>
</section>
