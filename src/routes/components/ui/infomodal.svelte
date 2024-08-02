<script lang="ts">
    import Terminal from "lucide-svelte/icons/terminal";
    import * as Alert from "$lib/components/ui/alert/index.js";

    export let infoTitle: string = "";
    export let infoDescription: string = "";
</script>

<Alert.Root>
    <Terminal class="h-4 w-4" />
    <Alert.Title class="font-semibold">{infoTitle}</Alert.Title>
    <Alert.Description>{infoDescription}</Alert.Description>
</Alert.Root>


