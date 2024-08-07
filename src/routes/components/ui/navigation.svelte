<script lang="ts">
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
    import { page } from '$app/stores';


    $: breadcrumb = getBreadcrumb($page.url.pathname);

    function getBreadcrumb(url: string) {
        const parts = url.split("/").filter(Boolean);
        return parts.map((part, index) => ({
            name: capitalizeFirstLetter(part),
            href: '/' + parts.slice(0, index + 1).join('/')
        }));
    }

    function capitalizeFirstLetter(string: string) {
        return string.charAt(0).toUpperCase() + string.slice(1).toLowerCase();
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