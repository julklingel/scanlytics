<script lang="ts">
	import Card from "../components/ui/card.svelte";
	import ErrorModal from "../components/ui/errormodal.svelte";
	import { toast } from "svelte-sonner";
	import { invoke } from "@tauri-apps/api/core";



async function runONNX() {
  const image_path = "Users/julklingel/Library/Application Support/com.scanlytics.dev/saved_images/Image:c2cfeuydnkdcpb42fjeg.jpg"
  const response = await invoke("test_onnx_model", {
    imagePath: JSON.stringify(image_path),
    });
    console.log(response);
    let res_string = JSON.stringify(response)
    toast.success(res_string);
}

</script>

<h1 class=" my-4 text-4xl font-extrabold tracking-tight lg:text-5xl">Menu</h1>

<div class="flex flex-wrap gap-4">
	<Card
		title="Patienten"
		description="Verwalte deine Patienten"
		link={"./patients"}
	/>
	<Card
		title="Befunde"
		description="Erstelle einen Befund"
		link={"./reports"}
	/>
	<Card
		title="Notizen"
		description="Verwalte deine Notizen"
		link={"./notes"}
	/>
	<Card
		title="Rechnungen"
		description="Erstelle eine Rechung"
		link={"./invoices"}
	/>


<button class=" p-20 bg-green-800 rounded-3xl text-yellow-50 shadow-2xl" on:click={runONNX}>ONNX</button>
</div>


