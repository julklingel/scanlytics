<script lang="ts">
  import DataTableActions from "./data-table-actions.svelte";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import PlusIcon from "lucide-svelte/icons/plus";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { writable, derived } from "svelte/store";
  import InfoMsg from "../../components/ui/infomodal.svelte";
  import { ReportStore } from "../../../stores/Report"; 
  import { getReports } from "../api/report-data";
  

  let filteredReports: any;
  const infoTitle: string | null | never = "No reports found";
  const infoDescription: string | null | never =
    "There are no reports available. Please add a new report.";
  let dataAvailable: boolean = false;
  const filterValue = writable("");

  const isReportStoreEmpty = derived(
    ReportStore,
    ($ReportStore) => $ReportStore.length === 0,
  );
  $: dataAvailable = !$isReportStoreEmpty;

  onMount(async () => {
    try {
      await getReports();
    } catch (error) {
      console.error(error);
    }
  });

  $: filteredReports = $filterValue
    ? $ReportStore.filter(
        (report) =>
          (report.patient?.name || "")
            .toLowerCase()
            .includes($filterValue.toLowerCase()) ||
          (report.condition || "")
            .toLowerCase()
            .includes($filterValue.toLowerCase()) ||
          (report.bodyType || "")
            .toLowerCase()
            .includes($filterValue.toLowerCase()),
      )
    : $ReportStore;

    $: console.log(filteredReports);

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
  }

  function handleCreateNewReport() {
    goto("/reports/new");
  }

  function handleReportView(id: string) {
    goto(`/reports/${id}`);
  }
</script>

{#if !dataAvailable}
  <div class="flex flex-col gap-4">
    <InfoMsg {infoTitle} {infoDescription} />
    <Button on:click={handleCreateNewReport}>
      <PlusIcon size={16} />
      <span>New Report</span>
    </Button>
  </div>
{:else}
  <div>
    <div class="flex gap-4 mb-2">
      <Button on:click={handleCreateNewReport}>
        <PlusIcon size={16} />
        <span>New Report</span>
      </Button>
      <Input
        class="max-w-sm"
        placeholder="Filter reports..."
        type="text"
        bind:value={$filterValue}
      />
    </div>
    <Table.Table>
      <Table.TableHeader>
        <Table.TableRow>
          <Table.TableHead>Patient</Table.TableHead>
          <Table.TableHead>Body Type</Table.TableHead>
          <Table.TableHead>Condition</Table.TableHead>
          <Table.TableHead>Created At</Table.TableHead>
          <Table.TableHead>Actions</Table.TableHead>
        </Table.TableRow>
      </Table.TableHeader>
      <Table.TableBody>
        {#each filteredReports as report}
          <Table.TableRow>
            <Table.TableCell on:click={() => handleReportView(report.id.id.String)}
              >{report.patient?.name || "N/A"}</Table.TableCell
            >
            <Table.TableCell on:click={() => handleReportView(report.id.id.String)}
              >{report.body_type || "N/A"}</Table.TableCell
            >
            <Table.TableCell on:click={() => handleReportView(report.id.id.String)}
              >{report.condition || "N/A"}</Table.TableCell
            >
            <Table.TableCell on:click={() => handleReportView(report.id.id.String)}
              >{formatDate(report.createdAt)}</Table.TableCell
            >
            <Table.TableCell>
              <DataTableActions id={report.id} patientId={report.patient?.id} />
            </Table.TableCell>
          </Table.TableRow>
        {/each}
      </Table.TableBody>
    </Table.Table>
  </div>
{/if}