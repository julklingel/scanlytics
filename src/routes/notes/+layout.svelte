<script lang="ts">
  import type { LayoutData } from "../menu/$types";
  import Navigation from "../components/ui/navigation.svelte";
  import AuthService from "../../stores/Auth";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  let isAuthenticated = false;

  onMount(async () => {
    AuthService.subscribe((value) => {
      isAuthenticated = value.isValidated;
    });

    try {
      await AuthService.validate();
    } catch (error) {
      console.error("Authentication failed:", error);
      goto("auth/login");
    }
  });
</script>

<div class="container mx-auto py-3">
  <Navigation />
  {#if isAuthenticated}
    <slot></slot>
  {:else}
    <p>Loading...</p>
  {/if}
</div>
