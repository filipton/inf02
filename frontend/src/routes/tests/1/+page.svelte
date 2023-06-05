<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import OneQuestionTest from "$lib/components/OneQuestionTest.svelte";

    let questionsBase: Question[];

    questions.subscribe(async (x) => {
        if (!browser || questionsBase) return;
        questionsBase = x;
    });
</script>

{#if questionsBase}
    <OneQuestionTest base={questionsBase} />
{/if}
