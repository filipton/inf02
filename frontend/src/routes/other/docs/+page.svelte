<script lang="ts">
    import { page } from "$app/stores";
    import Markdown from "$lib/components/Markdown/Markdown.svelte";
    import { docs } from "$lib/stores";

    let file = "README.md";
    let shownMarkdown: string = "";

    $: {
        file = $page.url.searchParams.get("v") ?? "README.md";
        docs.subscribe((d) => {
            if (d) {
                const entry = d.find((e) => e.url === file);
                if (entry) {
                    shownMarkdown = entry.markdown;

                    if ($page.url.hash) {
                        setTimeout(() => {
                            const element = document.getElementById(
                                $page.url.hash.slice(1)
                            );
                            if (element) {
                                element.scrollIntoView({
                                    behavior: "smooth",
                                });
                            }
                        }, 100);
                    }
                } else {
                    shownMarkdown = `# Docs not found: ${file}`;
                }
            }
        });
    }
</script>

<div class="h-full max-w-2xl mx-auto mt-4 px-2">
    <Markdown input={shownMarkdown} />
</div>
