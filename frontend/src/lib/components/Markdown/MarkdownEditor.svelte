<script lang="ts">
    import Markdown from "./Markdown.svelte";

    let showPreview = false;
    let input = "";
</script>

<div class="w-full h-full bg-black">
    <nav class="flex justify-center">
        <button
            class="hover:bg-gray-800 text-white font-bold py-2 px-4 rounded-l {showPreview
                ? 'bg-gray-900'
                : 'bg-gray-800'}"
            on:click={() => (showPreview = false)}
        >
            Edit
        </button>
        <button
            class="hover:bg-gray-800 text-white font-bold py-2 px-4 rounded-r {showPreview
                ? 'bg-gray-800'
                : 'bg-gray-900'}"
            on:click={() => (showPreview = true)}
        >
            Preview
        </button>
    </nav>

    {#if showPreview}
        <Markdown {input} />
    {:else}
        <textarea
            class="w-full h-full bg-transparent text-white resize-none outline-none"
            bind:value={input}
        />
    {/if}
</div>
