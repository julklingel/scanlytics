<script lang="ts">
    import Terminal from "lucide-svelte/icons/terminal";
    import * as Alert from "$lib/components/ui/alert/index.js";
    import CircleAlert from "lucide-svelte/icons/circle-alert";


    export let title: string = "";
    export let description: string = "";
</script>



   
  <Alert.Root variant="destructive">
    <CircleAlert class="h-4 w-4" />
    <Alert.Title class=" font-semibold">{title}</Alert.Title>
    <Alert.Description>{description}</Alert.Description>
  </Alert.Root>
  
  