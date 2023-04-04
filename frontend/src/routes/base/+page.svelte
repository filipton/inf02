<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import QuestionElement from "$lib/components/QuestionElement.svelte";

    let allQuestions: Question[] = [];

    questions.subscribe(async (x) => {
        if (!browser) return;

        allQuestions = x;
    });
</script>

<QuestionsHandler>
    {#each allQuestions as question, i}
        <QuestionElement
            {question}
            questionNumber={i + 1}
            ended={true}
            showDidntChoose={false}
        />
    {/each}
</QuestionsHandler>
