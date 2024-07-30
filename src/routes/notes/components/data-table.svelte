<script lang="ts">
  import {
    Render,
    Subscribe,
    createRender,
    createTable
  } from "svelte-headless-table";
  import {
    addHiddenColumns,
    addPagination,
    addSelectedRows,
    addSortBy,
    addTableFilter
  } from "svelte-headless-table/plugins";
  import { readable } from "svelte/store";
  import ArrowUpDown from "lucide-svelte/icons/arrow-up-down";
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import DataTableActions  from "./data-table-actions.svelte";
import DataTableCheckbox from "./data-table-checkbox.svelte";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { cn } from "$lib/utils.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import PlusIcon from "lucide-svelte/icons/plus";





   
type PatientNote = {
  id: string;
  patientName: string;
  patientId: string;
  symptoms: string;
  diagnosis: string;
  treatment: string;
  createdAt: string;
  updatedAt: string;
  followUpDate: string;
  severity: 'Low' | 'Medium' | 'High';
  isUrgent: boolean;
  department: string;
  attendingDoctor: string;
}

function handleCreateNewNote() {
        alert('Create');

    }

export const samplePatientNotes: PatientNote[] = [
  {
    id: "728ed52f",
    patientName: "John Doe",
    patientId: "JD001",
    symptoms: "Persistent cough, fever, and fatigue for 5 days",
    diagnosis: "Suspected upper respiratory infection",
    treatment: "Prescribed antibiotics and rest for 7 days",
    createdAt: "2023-07-15T10:30:00Z",
    updatedAt: "2023-07-15T11:45:00Z",
    followUpDate: "2023-07-22",
    severity: "Medium",
    isUrgent: false,
    department: "General Practice",
    attendingDoctor: "Dr. Smith"
  },
  {
    id: "93ab2c1e",
    patientName: "Jane Smith",
    patientId: "JS002",
    symptoms: "Severe abdominal pain, nausea, and vomiting",
    diagnosis: "Suspected appendicitis",
    treatment: "Scheduled for emergency appendectomy",
    createdAt: "2023-07-16T08:15:00Z",
    updatedAt: "2023-07-16T09:30:00Z",
    followUpDate: "2023-07-23",
    severity: "High",
    isUrgent: true,
    department: "Surgery",
    attendingDoctor: "Dr. Johnson"
  },
  {
    id: "5f4e9d3a",
    patientName: "Robert Brown",
    patientId: "RB003",
    symptoms: "Joint pain, stiffness, and swelling in multiple joints",
    diagnosis: "Rheumatoid Arthritis",
    treatment: "Prescribed NSAIDs and referred to rheumatologist",
    createdAt: "2023-07-14T20:00:00Z",
    updatedAt: "2023-07-17T15:30:00Z",
    followUpDate: "2023-08-14",
    severity: "Medium",
    isUrgent: false,
    department: "Rheumatology",
    attendingDoctor: "Dr. Davis"
  },
  {
    id: "2c7db6f1",
    patientName: "Emily Wilson",
    patientId: "EW004",
    symptoms: "Shortness of breath, chest pain, and rapid heartbeat",
    diagnosis: "Possible angina, further tests required",
    treatment: "Prescribed nitroglycerin, scheduled stress test",
    createdAt: "2023-07-10T07:00:00Z",
    updatedAt: "2023-07-10T08:30:00Z",
    followUpDate: "2023-07-17",
    severity: "High",
    isUrgent: true,
    department: "Cardiology",
    attendingDoctor: "Dr. Lee"
  },
  {
    id: "8e1a7b9c",
    patientName: "Michael Taylor",
    patientId: "MT005",
    symptoms: "Headache, sensitivity to light, and blurred vision",
    diagnosis: "Migraine",
    treatment: "Prescribed triptans and recommended lifestyle changes",
    createdAt: "2023-07-12T22:45:00Z",
    updatedAt: "2023-07-18T09:15:00Z",
    followUpDate: "2023-08-12",
    severity: "Low",
    isUrgent: false,
    department: "Neurology",
    attendingDoctor: "Dr. Martinez"
  }
];
const table = createTable(readable(samplePatientNotes), {
    sort: addSortBy({ disableMultiSort: true }),
    page: addPagination(),
    filter: addTableFilter({
      fn: ({ filterValue, value }) => value.toLowerCase().includes(filterValue.toLowerCase())
    }),
    select: addSelectedRows(),
    hide: addHiddenColumns()
  });
 
  const columns = table.createColumns([
    table.column({
      header: (_, { pluginStates }) => {
        const { allPageRowsSelected } = pluginStates.select;
        return createRender(DataTableCheckbox, {
          checked: allPageRowsSelected
        });
      },
      accessor: "id",
      cell: ({ row }, { pluginStates }) => {
        const { getRowState } = pluginStates.select;
        const { isSelected } = getRowState(row);
 
        return createRender(DataTableCheckbox, {
          checked: isSelected
        });
      },
      plugins: {
        sort: { disable: true },
        filter: { exclude: true }
      }
    }),
    table.column({
      header: "Patient Name",
      accessor: "patientName",
      plugins: {
        filter: {
          getFilterValue(value) {
            return value.toLowerCase();
          }
        }
      }
    }),
    table.column({
      header: "Patient ID",
      accessor: "patientId",
    }),
    table.column({
      header: "Symptoms",
      accessor: "symptoms",
      cell: ({ value }) => value.substring(0, 30) + (value.length > 30 ? '...' : ''),
    }),
    table.column({
      header: "Diagnosis",
      accessor: "diagnosis",
      cell: ({ value }) => value.substring(0, 30) + (value.length > 30 ? '...' : ''),
    }),
    table.column({
      header: "Severity",
      accessor: "severity",
    }),
    table.column({
      header: "Urgent",
      accessor: "isUrgent",
      cell: ({ value }) => value ? 'Yes' : 'No',
    }),
    table.column({
      header: "Created At",
      accessor: "createdAt",
      cell: ({ value }) => new Date(value).toLocaleString(),
    }),
    table.column({
      header: "",
      accessor: ({ id }) => id,
      cell: (item) => {
        return createRender(DataTableActions, { id: item.value });
      },
      plugins: {
        sort: { disable: true }
      }
    })
  ]);
 
  const {
    headerRows,
    pageRows,
    tableAttrs,
    tableBodyAttrs,
    flatColumns,
    pluginStates,
    rows
  } = table.createViewModel(columns);
 
  const { sortKeys } = pluginStates.sort;
 
  const { hiddenColumnIds } = pluginStates.hide;
  const ids = flatColumns.map((c) => c.id);
  let hideForId = Object.fromEntries(ids.map((id) => [id, true]));
 
  $: $hiddenColumnIds = Object.entries(hideForId)
    .filter(([, hide]) => !hide)
    .map(([id]) => id);
 
  const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
  const { filterValue } = pluginStates.filter;
 
  const { selectedDataIds } = pluginStates.select;
 
  const hideableCols = ["patientId", "symptoms", "diagnosis", "severity", "isUrgent", "createdAt"];
</script>
 
<div class="w-full">
  <div class="flex gap-4 items-center py-4">
    <Button on:click={handleCreateNewNote} class="flex items-center gap-2">
        <PlusIcon class="w-4 h-4" />
        New Note
    </Button>
    <Input
      class="max-w-sm"
      placeholder="Filter patient names..."
      type="text"
      bind:value={$filterValue}
    />
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild let:builder>
        <Button variant="outline" class="ml-auto" builders={[builder]}>
          Columns <ChevronDown class="ml-2 h-4 w-4" />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        {#each flatColumns as col}
          {#if hideableCols.includes(col.id)}
            <DropdownMenu.CheckboxItem bind:checked={hideForId[col.id]}>
              {col.header}
            </DropdownMenu.CheckboxItem>
          {/if}
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>
  <div class="rounded-md border">
    <Table.Root {...$tableAttrs}>
      <Table.Header>
        {#each $headerRows as headerRow}
          <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
              {#each headerRow.cells as cell (cell.id)}
                <Subscribe
                  attrs={cell.attrs()}
                  let:attrs
                  props={cell.props()}
                  let:props
                >
                  <Table.Head
                    {...attrs}
                    class={cn("[&:has([role=checkbox])]:pl-3")}
                  >
                    {#if cell.id === "patientName"}
                      <Button variant="ghost" on:click={props.sort.toggle}>
                        <Render of={cell.render()} />
                        <ArrowUpDown
                          class={cn(
                            $sortKeys[0]?.id === cell.id && "text-foreground",
                            "ml-2 h-4 w-4"
                          )}
                        />
                      </Button>
                    {:else}
                      <Render of={cell.render()} />
                    {/if}
                  </Table.Head>
                </Subscribe>
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Header>
      <Table.Body {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
          <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
            <Table.Row
              {...rowAttrs}
              data-state={$selectedDataIds[row.id] && "selected"}
            >
              {#each row.cells as cell (cell.id)}
            
                <Subscribe attrs={cell.attrs()} let:attrs>
                  <Table.Cell class="[&:has([role=checkbox])]:pl-3" {...attrs}>
                    <a href="./notes/{cell.id}">
                    {#if cell.id === "severity"}
                      <div class="capitalize font-medium">
                        <Render of={cell.render()} />
                      </div>
                    {:else if cell.id === "isUrgent"}
                      <div class={cell.value ? "text-red-500 font-bold" : ""}>
                        <Render of={cell.render()} />
                      </div>
                    {:else}
                      <Render of={cell.render()} />
                    {/if}
                </a>
                  </Table.Cell> 
                </Subscribe>
                
              {/each}
            </Table.Row>
          </Subscribe>
        {/each}
      </Table.Body>
    </Table.Root>
  </div>
  <div class="flex items-center justify-end space-x-2 py-4">
    <div class="text-muted-foreground flex-1 text-sm">
      {Object.keys($selectedDataIds).length} of {$rows.length} note(s) selected.
    </div>
    <Button
      variant="outline"
      size="sm"
      on:click={() => ($pageIndex = $pageIndex - 1)}
      disabled={!$hasPreviousPage}>Previous</Button
    >
    <Button
      variant="outline"
      size="sm"
      disabled={!$hasNextPage}
      on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
    >
  </div>
</div>