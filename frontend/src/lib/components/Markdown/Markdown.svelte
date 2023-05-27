<script lang="ts">
    import "./github.css";
    import { marked } from "marked";
    import { gfmHeadingId } from "marked-gfm-heading-id";
    import { baseUrl } from "marked-base-url";

    export let input = "";
    marked.use(gfmHeadingId());
    marked.use({
        pedantic: false,
        gfm: true,
    });
    marked.use(baseUrl("http://base.url"));
</script>

<article class="markdown-body">
    {@html marked(input, { mangle: false }).replaceAll(
        "http://base.url/",
        "?v="
    )}
</article>
