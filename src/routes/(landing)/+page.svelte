<script lang="ts">
    import HeaderSection from "../(landing)/components/header-section.svelte";
    import TitleSection from "../(landing)/components/title-section.svelte";
    import { invoke } from "@tauri-apps/api/core";

    interface TestdbResponse {
        id: string;
        bool1: boolean;
        txt1: string;
    }

    let writeId = "";
    let bool1 = false;
    let txt1 = "";
    let readId = "";
    let writeResult: TestdbResponse | null = null;
    let readResult: TestdbResponse | null = null;

    async function testDbWrite() {
        try {
            writeResult = await invoke("test_db_write", { id: writeId, bool1, txt1 });
            console.log("Write result:", writeResult);
        } catch (error) {
            console.error("Write Error:", error);
        }
    }

    async function testDbRead() {
        try {
            readResult = await invoke("test_db_read", { id: readId });
            console.log("Read result:", readResult);
        } catch (error) {
            console.error("Read Error:", error);
        }
    }

	let deleteId = "";
    let deleteResult: TestdbResponse | null = null;

    async function testDbDelete() {
        try {
            deleteResult = await invoke("test_db_delete", { id: deleteId });
            console.log("Delete result:", deleteResult);
        } catch (error) {
            console.error("Delete Error:", error);
        }
    }
</script>

<HeaderSection />
<TitleSection />

<h2>Write to DB</h2>
<input bind:value={writeId} placeholder="Enter ID for writing" />
<input type="checkbox" bind:checked={bool1} />
<input bind:value={txt1} placeholder="Enter text" />
<button on:click={testDbWrite}>Write to DB</button>

{#if writeResult}
    <h3>Write Result:</h3>
    <pre>{JSON.stringify(writeResult, null, 2)}</pre>
{/if}

<h2>Read from DB</h2>
<input bind:value={readId} placeholder="Enter ID for reading" />
<button on:click={testDbRead}>Read from DB</button>

{#if readResult}
    <h3>Read Result:</h3>
    <pre>{JSON.stringify(readResult, null, 2)}</pre>
{/if}

<h2>Delete from DB</h2>
<input bind:value={deleteId} placeholder="Enter ID to delete" />
<button on:click={testDbDelete}>Delete from DB</button>

{#if deleteResult !== null}
    <h3>Delete Result:</h3>
    {#if deleteResult}
        <pre>{JSON.stringify(deleteResult, null, 2)}</pre>
    {:else}
        <p>No record found to delete</p>
    {/if}
{/if}

