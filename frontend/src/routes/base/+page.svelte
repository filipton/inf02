<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import QuestionElement from "$lib/components/QuestionElement.svelte";

    let allQuestions: Question[] = [];
    let shownQuestions: Question[] = [];

    questions.subscribe(async (x) => {
        if (!browser) return;

        allQuestions = x;
        lazyLoad();
    });

    function onScroll() {
        if (shownQuestions.length >= allQuestions.length) return;
        let fromBottom =
            document.documentElement.scrollHeight -
            window.innerHeight -
            window.scrollY;

        if (fromBottom < 10000) {
            lazyLoad();
        }
    }

    function lazyLoad(count: number = 50) {
        let questions = allQuestions.slice(0, shownQuestions.length + count);
        shownQuestions = questions;
    }
</script>

<svelte:window on:scroll={onScroll} />

<QuestionsHandler>
    {#if shownQuestions.length < allQuestions.length}
        <button
            class="w-full h-12 bg-gray-600 text-white mb-8 rounded"
            on:click={() => lazyLoad(2227)}
        >
            LOAD EVERYTHING (HUGE LAG)
        </button>
    {/if}

    {#each shownQuestions as question, i}
        <QuestionElement
            {question}
            questionNumber={i + 1}
            ended={true}
            showDidntChoose={false}
        />
    {/each}
</QuestionsHandler>
