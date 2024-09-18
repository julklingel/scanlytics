<script lang="ts">
    import { Label } from "$lib/components/ui/label/index.js";
    import Button from "$lib/components/ui/button/button.svelte";
    import { toast } from "svelte-sonner";
    import { Input } from "$lib/components/ui/input/index.js";
  
    function handleSubmit(event: Event) {
      event.preventDefault();
      toast.success("Signup successful!");
    }
  </script>
  
  <div class="min-h-screen flex flex-col md:flex-row">
    <div class="w-1/2 bg-gray-50 dark:bg-slate-900 shadow-lg flex items-end justify-center">
      <img
        src="/login-img.png"
        alt="Signup illustration"
        class="max-w-full h-auto max-h-[85vh] object-contain"
      />
    </div>
  
    <div class="w-full md:w-1/2 flex flex-col justify-center p-8">
      <h1
        class="text-center text-4xl font-extrabold tracking-tight lg:text-5xl mb-8"
      >
        Sign Up
      </h1>
  
      <form on:submit={handleSubmit} class="space-y-4 max-w-md mx-auto w-full">
        <div class="space-y-2 w-full">
          <Label for="name" class="text-sm font-medium">Full Name</Label>
          <Input type="text" id="name" placeholder="Enter your full name" class="" />
        </div>
        <div class="space-y-2 w-full">
          <Label for="email" class="text-sm font-medium">Email</Label>
          <Input type="email" id="email" placeholder="Enter your email" class="" />
        </div>
        <div class="space-y-2">
          <Label for="password" class="text-sm font-medium">Password</Label>
          <Input type="password" id="password" placeholder="Create a password" class="" />
        </div>
        <div class="space-y-2">
          <Label for="confirm-password" class="text-sm font-medium">Confirm Password</Label>
          <Input type="password" id="confirm-password" placeholder="Confirm your password" class="" />
        </div>
        <Button type="submit" class="w-full">Sign Up</Button>
      </form>
  
      <div class="mt-6 text-center">
        <p class="text-sm text-gray-600">Already have an account?</p>
        <Button href="/auth/login" variant="link" class="">Login</Button>
      </div>
    </div>
  </div>
  