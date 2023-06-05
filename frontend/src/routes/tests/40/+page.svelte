<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import QuestionsTest from "$lib/components/QuestionsTest.svelte";

    let questionsBase: Question[];

    questions.subscribe(async (x) => {
        if (!browser || questionsBase) return;
        questionsBase = x;
    });
</script>

{#if questionsBase}
    <QuestionsTest key="q" base={questionsBase} testSize={40} />
{/if}
