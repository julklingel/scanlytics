<script lang="ts">
	import Card from "../components/ui/card.svelte";
	

</script>

<h1 class=" my-4 text-4xl font-extrabold tracking-tight lg:text-5xl">Menu</h1>

<div class="flex flex-wrap gap-4">
	<Card
		title="Patienten"
		description="Verwalte deine Patienten"
		link={"./patients"}
	/>
	<Card
		title="Befunde"
		description="Erstelle einen Befund"
		link={"./reports"}
	/>
	<Card
		title="Notizen"
		description="Verwalte deine Notizen"
		link={"./notes"}
	/>
	<Card
		title="Rechnungen"
		description="Erstelle eine Rechung"
		link={"./invoices"}
	/>

</div>


