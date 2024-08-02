<script>
import "../app.css";


import { ModeWatcher } from "mode-watcher";
import { Toaster } from "$lib/components/ui/sonner";
</script>




<main class="h-screen w-screen">
<ModeWatcher></ModeWatcher>
<Toaster />
<slot></slot>
</main>