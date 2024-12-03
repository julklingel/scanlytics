import { invoke } from "@tauri-apps/api/core";
import { toast } from "svelte-sonner";
import type { FileData, ModelResponse } from "$lib/types/report.types";

export async function processImages(
  files: File[],
  active_user: string,
  selectedModel: string
): Promise<ModelResponse | null> {
  try {
    const fileData: FileData[] = await Promise.all(
      files.map(async (file) => ({
        filename: file.name,
        extension: file.name.split(".").pop() || "",
        data: Array.from(new Uint8Array(await file.arrayBuffer())),
      }))
    );

    const result: ModelResponse = await invoke("process_images", {
      imageData: JSON.stringify(fileData),
      userName: JSON.stringify(active_user),
      modelName: JSON.stringify(selectedModel),
    });

    toast.success("Images processed successfully");
    return result;
  } catch (error) {
    console.error("Error processing images:", error);
    toast.error("Error processing images");
    return null;
  }
}
