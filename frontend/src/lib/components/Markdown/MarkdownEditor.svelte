<script lang="ts">
    import Markdown from "./Markdown.svelte";

    let input = "";
</script>

<div class="absolute top-0 left-0 z-10">
    <div class="flex flex-row w-screen h-screen bg-black">
        <div
            class="w-full h-full bg-transparent border-solid border-r-white border-r"
        >
            <textarea
                class="w-full h-full bg-transparent text-white"
                bind:value={input}
            />
        </div>
        <div
            class="w-full h-full bg-transparent border-solid border-l-white border-l"
        >
            <Markdown {input} />
        </div>
    </div>
</div>
