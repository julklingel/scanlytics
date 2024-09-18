<script lang="ts">
  import DarkMode from "../../components/ui/darkmode.svelte";
  import { Button } from "$lib/components/ui/button";
  import { goto } from "$app/navigation";

  async function navigate2menu() {
    await goto("/menu");
  }

  async function navigate2login() {
    await goto("/auth/login");
  }

  async function navigate2signup() {
    await goto("/auth/signup");
  }
</script>

<section class="flex flex-col h-screen items-center justify-center">
  <h1 class="text-4xl font-extrabold tracking-tight lg:text-5xl">Scanlytics</h1>
  <p class="mt-6 pl-6 italic">#1 Text Editor in der Medizin</p>
  <br />
  <div class="flex flex-row gap-6">
    <Button on:click={navigate2menu}>Directly to Menu - DEV</Button>
    <Button on:click={navigate2login}>Login</Button>
    <Button on:click={navigate2signup}>Signup</Button>
  </div>
</section>
