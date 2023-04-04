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

        const lastQuestion = document.querySelector(
            ".question:last-child"
        ) as HTMLElement;
        if (isInViewport(lastQuestion)) {
            lazyLoad();
        }
    }

    function lazyLoad(count: number = 100) {
        let questions = allQuestions.slice(0, shownQuestions.length + count);
        shownQuestions = questions;
    }

    function isInViewport(element: HTMLElement) {
        const rect = element.getBoundingClientRect();
        return (
            rect.top >= 0 &&
            rect.left >= 0 &&
            rect.bottom <=
                (window.innerHeight || document.documentElement.clientHeight) &&
            rect.right <=
                (window.innerWidth || document.documentElement.clientWidth)
        );
    }
</script>

<svelte:window on:scroll={onScroll} />

<QuestionsHandler>
    {#if shownQuestions.length < allQuestions.length}
        <button
            class="w-full h-12 bg-gray-600 text-white mb-8 rounded"
            on:click={() => lazyLoad(2227)}
        >
            LOAD EVERYTHING (BIG LAG)
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
