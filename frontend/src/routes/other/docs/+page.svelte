<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import Markdown from "$lib/components/Markdown/Markdown.svelte";
    import { docs } from "$lib/stores";

    let file = "README.md";
    let shownMarkdown: string = "";

    let fileHistory: string[] = [];

    $: {
        file = $page.url.searchParams.get("v") ?? "README.md";
        docs.subscribe((d) => {
            if (d) {
                const entry = d.find((e) => e.url === file);
                if (entry) {
                    if (fileHistory[fileHistory.length - 1] !== file) {
                        fileHistory.push(file);
                    }
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

    function back() {
        if (fileHistory.length > 1) {
            fileHistory.pop();
            goto(`/other/docs?v=${fileHistory[fileHistory.length - 1]}`);
        } else {
            goto("/other/docs");
        }
    }
</script>

<div class="h-full max-w-2xl mx-auto mt-4 px-2">
    {#if file !== "README.md"}
        <div class="mb-4">
            <button
                class="border-red-100 bg-gray-800 hover:bg-gray-600 text-white font-bold py-2 px-2 rounded"
                on:click={back}
            >
                &lt; Back
            </button>
        </div>
    {/if}

    <Markdown input={shownMarkdown} />
</div>
