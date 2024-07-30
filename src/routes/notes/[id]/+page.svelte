

<script lang="ts">
    import { onMount } from 'svelte';
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";

    export let noteId: string;

    type PatientNote = {
        id: string;
        patientName: string;
        patientId: string;
        symptoms: string;
        diagnosis: string;
        treatment: string;
        severity: 'Low' | 'Medium' | 'High';
        isUrgent: boolean;
        createdAt: string;
        updatedAt: string;
        attendingDoctor: string;
    };

    let note: PatientNote | null = null;
    let loading = true;
    let error = '';

    onMount(async () => {
        try {
            const response = await fetch(`/api/notes/${noteId}`);
            if (!response.ok) throw new Error('Failed to fetch note');
            note = await response.json();
        } catch (e) {
            error = e.message;
        } finally {
            loading = false;
        }
    });

    function formatDate(dateString: string) {
        return new Date(dateString).toLocaleString();
    }
</script>

<div class="container mx-auto p-4">
    {#if loading}
        <p>Loading note details...</p>
    {:else if error}
        <p class="text-red-500">Error: {error}</p>
    {:else if note}
        <Card.Root class="w-full max-w-3xl mx-auto">
            <Card.Header>
                <Card.Title class="text-2xl">{note.patientName}'s Medical Note</Card.Title>
                <Card.Description>Patient ID: {note.patientId}</Card.Description>
            </Card.Header>
            <Card.Content>
                <div class="grid grid-cols-2 gap-4">
                    <div>
                        <h3 class="font-bold">Symptoms:</h3>
                        <p>{note.symptoms}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Diagnosis:</h3>
                        <p>{note.diagnosis}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Treatment:</h3>
                        <p>{note.treatment}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Severity:</h3>
                        <p class={note.severity === 'High' ? 'text-red-500' : ''}>{note.severity}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Urgent:</h3>
                        <p class={note.isUrgent ? 'text-red-500 font-bold' : ''}>{note.isUrgent ? 'Yes' : 'No'}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Attending Doctor:</h3>
                        <p>{note.attendingDoctor}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Created At:</h3>
                        <p>{formatDate(note.createdAt)}</p>
                    </div>
                    <div>
                        <h3 class="font-bold">Last Updated:</h3>
                        <p>{formatDate(note.updatedAt)}</p>
                    </div>
                </div>
            </Card.Content>
            <Card.Footer class="flex justify-end space-x-2">
                <Button variant="outline" on:click={() => history.back()}>Back</Button>
                <Button on:click={() => alert('Edit functionality to be implemented')}>Edit Note</Button>
            </Card.Footer>
        </Card.Root>
    {:else}
        <p>No note found with ID: {noteId}</p>
    {/if}
</div>

<style>
    h3 {
        margin-bottom: 0.5rem;
    }
    p {
        margin-bottom: 1rem;
    }
</style>