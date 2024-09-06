<script lang="ts">
  import { page } from "$app/stores";
</script>

<section class="flex justify-center items-center w-full h-screen">
    <div class="py-6 bg-white rounded-lg shadow-md p-6">
      <h1 class=" font-bold">{$page.status}: {$page.error?.message}</h1>
      <p>
        {#if $page.status === 404}
          The page you were looking for could not be found.
        {:else}
          An error occurred.
        {/if}
      </p>
    </div>
  </section>
  