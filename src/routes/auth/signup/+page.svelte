<script lang="ts">
  import { onMount } from "svelte";
  import { Label } from "$lib/components/ui/label/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import { toast } from "svelte-sonner";
  import { Input } from "$lib/components/ui/input/index.js";
  import { goto } from "$app/navigation";
  import { mode } from "mode-watcher";
  import { invoke } from "@tauri-apps/api/core";
  import { Progress } from "$lib/components/ui/progress";

  let logoSrc: string;
  let isLoading = false;
  let progressValue = 0;

  $: logoSrc = $mode === "dark" ? "/logo-dark.png" : "/logo.png";

  let signupData = {
    full_name: "",
    email: "",
    password: "",
    confirm_password: "",
  };

  function navigate2landing() {
    goto("/");
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (!signupData.full_name || !signupData.email || !signupData.password || !signupData.confirm_password) {
      toast.error("Please fill in all fields.");
      return;
    }
    
    isLoading = true;
    progressValue = 0;

    const progressInterval = setInterval(() => {
      progressValue += 10;
      if (progressValue >= 90) {
        clearInterval(progressInterval);
      }
    }, 200);

    try {
      const response = await invoke("signup", {
        signupData: JSON.stringify(signupData),
      });
      progressValue = 100;
      toast.success("Signup successful!");
      setTimeout(() => {
        isLoading = false;
        goto("/menu");
      }, 500);
    } catch (error) {
      let stringerror = JSON.stringify(error);
      toast.error(stringerror);
      isLoading = false;
    } finally {
      clearInterval(progressInterval);
    }
  }
</script>

<div class="min-h-screen flex flex-col md:flex-row">
  <div
    class="w-1/2 bg-gray-50 dark:bg-slate-900 shadow-lg flex items-end justify-center"
  >
    <img
      src="/login-img.png"
      alt="Signup illustration"
      class="max-w-full h-auto max-h-[85vh] object-contain"
    />
  </div>

  <div class="w-full md:w-1/2 flex flex-col justify-center p-8">
    <div class="flex justify-center gap-3">
      <div class="">
        <button on:click={navigate2landing}>
          <img src={logoSrc} alt="Logo" class=" h-12 w-12" />
        </button>
      </div>

      <h1
        class="text-center text-4xl font-extrabold tracking-tight lg:text-5xl"
      >
        Sign Up
      </h1>
    </div>

    <form on:submit={handleSubmit} class="space-y-4 max-w-md mx-auto w-full">
      <div class="space-y-2 w-full">
        <Label for="name" class="text-sm font-medium">Full Name</Label>
        <Input
          type="text"
          id="name"
          placeholder="Enter your full name"
          class=""
          bind:value={signupData.full_name}
        />
      </div>
      <div class="space-y-2 w-full">
        <Label for="email" class="text-sm font-medium">Email</Label>
        <Input
          type="email"
          id="email"
          placeholder="Enter your email"
          class=""
          bind:value={signupData.email}
        />
      </div>
      <div class="space-y-2">
        <Label for="password" class="text-sm font-medium">Password</Label>
        <Input
          type="password"
          id="password"
          placeholder="Create a password"
          class=""
          bind:value={signupData.password}
        />
      </div>
      <div class="space-y-2">
        <Label for="confirm-password" class="text-sm font-medium"
          >Confirm Password</Label
        >
        <Input
          type="password"
          id="confirm-password"
          placeholder="Confirm your password"
          class=""
          bind:value={signupData.confirm_password}
        />
      </div>
      {#if isLoading}
        <Progress value={progressValue} max={100} class="w-full" />
      {/if}
      <Button type="submit" class="w-full" disabled={isLoading}>
        {isLoading ? 'Signing Up...' : 'Sign Up'}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <p class="text-sm text-gray-600">Already have an account?</p>
      <Button href="/auth/login" variant="link" class="">Login</Button>
    </div>
  </div>
</div>
