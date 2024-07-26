<script lang="ts">
    import { Button } from "$lib/components/ui/button";

    export let buttonText: string
    export let variant: "link" | "default" | "destructive" | "outline" | "secondary" | "ghost"
  </script>

  
   
  <Button  variant="{variant}" >{buttonText}</Button>