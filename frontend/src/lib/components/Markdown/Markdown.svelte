<script lang="ts">
    import { parse } from "marked";
    import "./github.css";

    export let input = "";
    $: htmlMarkdown = parse(input);
</script>

<article class="markdown-body">{@html htmlMarkdown}</article>
