<script lang="ts">
    import type { LayoutData } from './$types';
    import Navigation from '../components/ui/navigation.svelte'

    import { page } from '$app/stores';  

    let url:string = $page.url.pathname

</script>




<div class="container mx-auto py-3">
<Navigation/>
<slot>

</slot>
</div>