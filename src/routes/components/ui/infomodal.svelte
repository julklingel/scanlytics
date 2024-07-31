<script lang="ts">
    import Terminal from "lucide-svelte/icons/terminal";
    import * as Alert from "$lib/components/ui/alert/index.js";

    export let title: string = "";
    export let description: string = "";
</script>

<Alert.Root>
    <Terminal class="h-4 w-4" />
    <Alert.Title>{title}</Alert.Title>
    <Alert.Description>{description}</Alert.Description>
</Alert.Root>


