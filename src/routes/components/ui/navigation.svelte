<script lang="ts">
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";

    export let url: string;
    let breadcrumb = getBreadcrumb(url);

    function getBreadcrumb(url: string) {
        return url.split("/").map(capitalizeFirstLetter);
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
        {#if breadcrumb[1] !== "Menu"}
            <Breadcrumb.Separator />
            <Breadcrumb.Item>
                <Breadcrumb.Link href="/components"
                    >{breadcrumb[1]}</Breadcrumb.Link
                >
            </Breadcrumb.Item>
        {/if}
    </Breadcrumb.List>
</Breadcrumb.Root>