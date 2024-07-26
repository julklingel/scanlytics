<script lang="ts">
	import HeaderSection from '../(landing)/components/header-section.svelte';
	import TitleSection from '../(landing)/components/title-section.svelte';

	import { invoke } from '@tauri-apps/api/core';

	// Invoke example
	async function callcrabby() {
		invoke('crabby_print');
	}

	

</script>

<HeaderSection />
<TitleSection />







