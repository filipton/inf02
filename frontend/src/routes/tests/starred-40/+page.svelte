<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import QuestionsTest from "$lib/components/QuestionsTest.svelte";

    let base: Question[];
    questions.subscribe((x) => {
        if (base || !x) return;

        base = x.filter((x) => x.starred);
    });
</script>

{#if base}
    <QuestionsTest key="s" {base} testSize={40} />
{/if}
