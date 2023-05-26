<script lang="ts">
    import { onMount } from "svelte";
    import { Converter } from "showdown";

    let htmlMarkdown = "";

    onMount(() => {
        let input =
            "| First Header  | Second Header |\n" +
            "| ------------- | ------------- |\n" +
            "| Content Cell  | Content Cell  |\n" +
            "| Content Cell  | Content Cell  |\n - [ ] test\n - [x] test\n";

        var converter = new Converter();
        converter.setOption("tables", true);
        converter.setOption("tasklists", true);
        htmlMarkdown = converter.makeHtml(input);
    });
</script>

<div class="mt-6 w-full max-w-2xl mx-auto text-white text-center px-2">
    <div class="mb-6">
        <h1 class="text-2xl font-bold">Normalne</h1>

        <a
            href="/other/tests/1"
            class="block w-full bg-gray-900 py-2 px-4 rounded my-2 hover:bg-gray-800"
        >
            Test 1 Pytanie
        </a>

        <a
            href="/other/tests/40"
            class="block w-full bg-gray-900 py-2 px-4 rounded my-2 hover:bg-gray-800"
        >
            Test 40 Pytan
        </a>
    </div>

    <div class="mb-6">
        <h1 class="text-2xl font-bold">Z gwiazdkami</h1>

        <a
            href="/other/tests/starred-1"
            class="block w-full bg-gray-900 py-2 px-4 rounded my-2 hover:bg-gray-800"
        >
            Test 1 Pytanie
        </a>

        <a
            href="/other/tests/starred-40"
            class="block w-full bg-gray-900 py-2 px-4 rounded my-2 hover:bg-gray-800"
        >
            Test 40 Pytan
        </a>

        <a
            href="/other/tests/starred-all"
            class="block w-full bg-gray-900 py-2 px-4 rounded my-2 hover:bg-gray-800"
        >
            Test Wszystkie
        </a>
    </div>
</div>

<article class="prose prose-invert">{@html htmlMarkdown}</article>
