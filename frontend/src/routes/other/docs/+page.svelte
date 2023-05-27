<script lang="ts">
    import { browser } from "$app/environment";
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import MarkdownEditor from "$lib/components/Markdown/MarkdownEditor.svelte";

    $: file = $page.url.searchParams.get("v") ?? "/";
    $: {
        if (browser) {
            if ($page.url.searchParams.get("v") !== file) {
                $page.url.searchParams.set("v", file);
                goto(
                    `?${decodeURIComponent($page.url.searchParams.toString())}`
                );
            }
        }
    }
</script>

<div class="h-screen max-w-2xl mx-auto">
    <MarkdownEditor />
</div>
