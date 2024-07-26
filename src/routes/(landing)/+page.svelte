<script lang="ts">
	import HeaderSection from '../(landing)/components/header-section.svelte';
	import TitleSection from '../(landing)/components/title-section.svelte';
	import Card from '../../components/ui/card.svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	




	let newRecord = { id: '', name: '' };
	let result = '';

	async function crappyCall() {
		console.log('Grias di')
		invoke('crabby_print');

	}

	async function createRecord() {
		try {
			const jsonString = JSON.stringify(newRecord);
			result = await invoke('create_record', { data: jsonString });
			console.log('Record created:', result);
		} catch (error) {
			console.error('Error creating record:', error);
		}
	}
</script>

<HeaderSection />
<TitleSection />
<br />
<input bind:value={newRecord.id} placeholder="ID" />
<input bind:value={newRecord.name} placeholder="Name" />
<Button on:click={createRecord}>Create Record</Button>
<Button on:click={crappyCall}>Crab crab crab</Button>


{#if result}
	<p>Created record: {result}</p>
{/if}
