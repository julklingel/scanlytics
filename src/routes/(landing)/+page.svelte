<script lang="ts">
	import HeaderSection from "../(landing)/components/header-section.svelte";
	import TitleSection from "../(landing)/components/title-section.svelte";

	import { invoke } from "@tauri-apps/api/core";

	interface TestdbResponse {
		id: string;
		bool1: boolean;
		txt1: string;
	}

	let id = "";
	let bool1 = false;
	let txt1 = "";
	let result: TestdbResponse | null = null;

	async function testDb() {
		try {
			result = await invoke("test_db", { id, bool1, txt1 });
			console.log(result);
		} catch (error) {
			console.error("Error:", error);
		}
	}
</script>

<HeaderSection />
<TitleSection />


<input bind:value={id} placeholder="Enter ID" />
<input type="checkbox" bind:checked={bool1} />
<input bind:value={txt1} placeholder="Enter text" />

<button on:click={testDb}>Test DB</button>

{#if result}
	<pre>{JSON.stringify(result, null, 2)}</pre>
{/if}
