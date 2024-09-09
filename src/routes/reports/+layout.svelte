<script lang="ts">
    import type { LayoutData } from './$types';
    import Navigation from '../components/ui/navigation.svelte'

    import { page } from '$app/stores';  


</script>




<div class="container mx-auto py-3">
<Navigation/>
<slot>

</slot>
</div>