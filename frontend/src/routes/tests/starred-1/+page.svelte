<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import OneQuestionTest from "$lib/components/OneQuestionTest.svelte";

    let base: Question[];
    questions.subscribe((x) => {
        if (base || !x) return;

        base = x.filter((x) => x.starred);
    });
</script>

{#if base}
    <OneQuestionTest {base} />
{/if}
