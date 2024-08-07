<script lang="ts">
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
    import { page } from "$app/stores";
    import { PatientStore } from "../../../stores/Patient";
    import { UserStore } from "../../../stores/User";
    import { patientNotes } from "../../../stores/PatientNote";

    function convertIdToName(id: string, store: any) {
        const item = store.find((item: any) => item.id === id);
        return item ? item.name : "Unknown";
    }

    function convertIdtoPateintName(id: string, store: any) {
        const item = store.find((item: any) => item.id === id);
        console.log("Item", item);
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

        if (breadcrumb[0].name === "Patients") {
            if (parts.length > 1) {
                breadcrumb[1].name = convertIdToName(parts[1], $PatientStore);
            }
        } else if (breadcrumb[0].name === "Notes") {
            if (parts.length > 1) {
                console.log("Back at it again", parts[1]);
                breadcrumb[1].name = convertIdtoPateintName(
                    parts[1],
                    $patientNotes,
                );
            }
        } else if (breadcrumb[0].name === "Users") {
            if (parts.length > 1) {
                breadcrumb[1].name = convertIdToName(parts[1], $UserStore);
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
