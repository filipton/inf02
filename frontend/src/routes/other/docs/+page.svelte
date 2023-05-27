<script lang="ts">
    import { page } from "$app/stores";
    import MarkdownEditor from "$lib/components/Markdown/MarkdownEditor.svelte";
    import Markdown from "$lib/components/Markdown/Markdown.svelte";
    import { docs } from "$lib/stores";

    let editMode: boolean = false;

    let file = "";
    let shownMarkdown: string = "";

    $: {
        file = $page.url.searchParams.get("v") ?? "";
        docs.subscribe((d) => {
            if (d) {
                const entry = d.find((e) => e.url === file);
                if (entry) {
                    shownMarkdown = entry.markdown;
                } else {
                    shownMarkdown = `# Docs not found: ${file}`;
                }
            }
        });
    }
</script>

<div class="h-screen max-w-2xl mx-auto mt-4">
    {#if editMode}
        <MarkdownEditor />
    {:else}
        <Markdown input={shownMarkdown} />
    {/if}
</div>
