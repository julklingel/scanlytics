<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    
    export let report_text: string = "";
    export let onRevert: () => void;
  </script>
  
  <div class="flex flex-col h-full p-4">
    <div class="flex justify-between items-center py-2">
      <h2 class="text-lg font-semibold">Befund</h2>
      <Button variant="secondary" on:click={onRevert}>Revert</Button>
    </div>
    <textarea
      bind:value={report_text}
      class="flex-grow p-2 border rounded-md resize-none"
      placeholder="Schreiben Sie Ihren Befund hier..."
    />
  </div>
  