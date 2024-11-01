<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "svelte-sonner";

  let email = "";

  let isLoading = false;
  let progressValue = 0;

  let resetData = {
    user_name: "",
  };

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (!resetData.user_name) {
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
      const response = await invoke("reset_password", {
        resetData: JSON.stringify(resetData),
      });
      progressValue = 100;
      toast.success("Email sent successfully!");
      setTimeout(() => {
        isLoading = false;
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

<Dialog.Root>
  <Dialog.Trigger class={buttonVariants({ variant: "link" })}>
    <p class=" text-blue-600">Forgot password?</p>
  </Dialog.Trigger>
  <Dialog.Content class="sm:max-w-[425px]">
    <form on:submit={handleSubmit}>
      <Dialog.Header>
        <Dialog.Title>Forgot your Password?</Dialog.Title>
        <Dialog.Description>
          No worries! Just enter your email address and we'll send you a reset
          link.
        </Dialog.Description>
      </Dialog.Header>

      <div class="grid gap-4 py-4">
        <Label for="email">Email</Label>
        <Input
          type="email"
          id="email"
          placeholder="Enter your email"
          class=""
          bind:value={resetData.user_name}
        />
      </div>
      <Dialog.Footer>
        <Button type="submit">Send</Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
