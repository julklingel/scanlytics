<script lang="ts">
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
    import { page } from "$app/stores";
    import { PatientStore } from "../../../stores/Patient";
    import { UserStore } from "../../../stores/User";
    import { PatientNotesStore } from "../../../stores/PatientNote";
    import { ReportStore } from "../../../stores/Report";

    function convertIdToName(id: string, store: any) {
        const item = store.find((item: any) => item.id === id);
        return item ? item.name : "Unknown";
    }

    function convertIdtoPateintName(id: string, store: any) {
        const item = store.find((item: any) => item.id === id);
        let patientName = convertIdToName(item.patientId, $PatientStore);
        return patientName;
    }

    function capitalizeFirstLetter(string: string) {
        return string.charAt(0).toUpperCase() + string.slice(1).toLowerCase();
    }

    $: breadcrumb = getBreadcrumb($page.url.pathname);

    function getBreadcrumb(url: string) {
        const parts = url.split("/").filter(Boolean);
        let breadcrumb = parts.map((part, index) => ({
            name: capitalizeFirstLetter(part),
            href: "/" + parts.slice(0, index + 1).join("/"),
        }));

        if (parts.length > 1) {
            switch (breadcrumb[0].name) {
                case "Patients":
                    if (parts[1] === "new") {
                        breadcrumb[1].name = "New";
                    } else {
                        breadcrumb[1].name = convertIdToName(
                            parts[1],
                            $PatientStore,
                        );
                    }
                    break;
                case "Notes":
                    if (parts[1] === "new") {
                        breadcrumb[1].name = "New";
                    } else {
                        const note = $PatientNotesStore.find(
                            (note) => note.id.id.String === parts[1]
                        );
                        if (note) {
                            breadcrumb[1].name = note.patient.name;
                        } else {
                            breadcrumb[1].name = "Unknown Patient";
                        }
                    }
                    break;
                case "Users":
                    breadcrumb[1].name = convertIdToName(parts[1], $UserStore);
                    break;
                
                case "Reports":
                    if (parts[1] === "new") {
                        breadcrumb[1].name = "New";
                    } else {
                        const report = $ReportStore.find(
                            (report) => report.id.id.String === parts[1]
                        );
                        if (report) {
                            breadcrumb[1].name = report.patient.name;
                        } else {
                            breadcrumb[1].name = "Unknown Patient";
                        }
                    }
            }
        }

        return breadcrumb;
    }
</script>

<Breadcrumb.Root>
    <Breadcrumb.List>
        <Breadcrumb.Item>
            <Breadcrumb.Link href="/menu">Menu</Breadcrumb.Link>
        </Breadcrumb.Item>
        {#each breadcrumb as crumb, index}
            {#if index > 0 || crumb.name.toLowerCase() !== "menu"}
                <Breadcrumb.Separator />
                <Breadcrumb.Item>
                    <Breadcrumb.Link href={crumb.href}>
                        {crumb.name}
                    </Breadcrumb.Link>
                </Breadcrumb.Item>
            {/if}
        {/each}
    </Breadcrumb.List>
</Breadcrumb.Root>