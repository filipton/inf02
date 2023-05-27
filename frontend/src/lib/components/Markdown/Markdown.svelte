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

    const URL_REGEX = /"(http:\/\/base.url\/)(.*)"/g;
    function replaceUrls(content: string) {
        let urls = [...content.matchAll(URL_REGEX)];
        for (let url of urls) {
            if (url[2].startsWith("#")) {
                content = content.replace(url[0], url[2]);
            } else {
                content = content.replace(
                    url[0],
                    url[0].replace(url[1], "?v=")
                );
            }
        }

        return content;
    }
</script>

<article class="markdown-body">
    {@html replaceUrls(marked(input, { mangle: false }))}
</article>
