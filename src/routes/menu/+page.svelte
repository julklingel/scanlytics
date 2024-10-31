<script lang="ts">
	import Card from "../components/ui/card.svelte";
	import { LogOut } from "lucide-svelte";
	import Button from "$lib/components/ui/button/button.svelte";
	import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { toast } from "svelte-sonner";
</script>

<div class="flex justify-between items-center mb-6">
	<h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">Menu</h1>
	<Button variant={"ghost"}
		class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg transition-colors"
		on:click={() => {
			try {
				invoke("logout");
			} catch (error) {
				let errMsg = `Something went wrong: ${error}`;
				toast(errMsg);
			}

			 goto("/auth/login");
		}}
	>
		<LogOut size={20} />
		<span>Logout</span>
	</Button>
</div>

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
