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
  import ForgotPassword from "../components/forgot-password.svelte";
  import AuthService from "../../../stores/Auth";


  let logoSrc: string;
  let isLoading = false;
  let progressValue = 0;

  let loginData = {
    user_email: "",
    user_password: "",
  };

  $: logoSrc = $mode === "dark" ? "/logo-dark.png" : "/logo.png";

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (!loginData.user_email || !loginData.user_password) {
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
      const response = await invoke("login", {
        loginData: JSON.stringify(loginData),
      });
      AuthService.login(loginData.user_email);
      progressValue = 100;
      toast.success("Login successful!");
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

  function navigate2landing() {
    goto("/");
  }
</script>

<div class="min-h-screen flex flex-col md:flex-row">
  <div
    class="w-1/2 bg-gray-50 dark:bg-slate-900 shadow-lg flex items-end justify-center"
  >
    <img
      src="/login-img.png"
      alt="Login illustration"
      class="max-w-full h-auto max-h-[85vh] object-contain"
    />
  </div>

  <div class="w-full md:w-1/2 flex flex-col justify-center items-center p-8">
    <div class="flex justify-center gap-3">
      <div class="">
        <button on:click={navigate2landing}>
          <img src={logoSrc} alt="Logo" class=" h-12 w-12" />
        </button>
      </div>

      <h1
        class="text-center text-4xl font-extrabold tracking-tight lg:text-5xl"
      >
        Login
      </h1>
    </div>

    <form on:submit={handleSubmit} class="space-y-4 max-w-md mx-auto w-full">
      <div class="space-y-2 w-full">
        <Label for="email" class="text-sm font-medium">Email</Label>
        <Input
          type="email"
          id="email"
          placeholder="Enter your email"
          class=""
          bind:value={loginData.user_email}
        />
      </div>
      <div class="space-y-2">
        <Label for="password" class="text-sm font-medium">Password</Label>
        <Input
          type="password"
          id="password"
          placeholder="Enter your password"
          class=""
          bind:value={loginData.user_password}
        />
      </div>
      <div class="flex justify-end">
        <ForgotPassword />
      </div>
      {#if isLoading}
        <Progress value={progressValue} max={100} class="w-full" />
      {/if}
      <Button type="submit" class="w-full" disabled={isLoading}>
        {isLoading ? "Logging in..." : "Login"}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <p class="text-sm text-gray-600">Don't have an account?</p>
      <Button href="/auth/signup" variant="link" class="">Sign Up</Button>
    </div>
  </div>
</div>
