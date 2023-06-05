<script lang="ts">
    import type { Question } from "$lib/types";
    import QuestionElement from "$lib/components/QuestionElement.svelte";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import { shuffleArray } from "$lib/utils";
    import { onMount } from "svelte";

    export let base: Question[];
    export let starEnabled: boolean = true;

    let questionsPool: Question[];
    let shownQuestion: Question;

    let selected = false;

    onMount(async () => {
        await getNextQuestion();
    });

    async function getNextQuestion() {
        selected = false;
        if (!questionsPool || questionsPool.length == 0) {
            questionsPool = JSON.parse(JSON.stringify(base));
        }

        let randomId = Math.floor(Math.random() * questionsPool.length);
        shownQuestion = questionsPool[randomId];
        questionsPool = removeAt(questionsPool, randomId);

        await scrambleAnwsers();
    }

    function removeAt(array: any[], index: number) {
        return array.filter((_, i) => i !== index);
    }

    async function scrambleAnwsers() {
        let old = shownQuestion.anwsers[shownQuestion.correct - 1];
        shuffleArray(shownQuestion.anwsers);

        shownQuestion.correct =
            shownQuestion.anwsers.findIndex((x) => x == old) + 1;
    }

    async function onKeyDown(e: KeyboardEvent) {
        switch (e.key) {
            case "1":
                shownQuestion.selected = 1;
                selected = true;
                return;
            case "2":
                shownQuestion.selected = 2;
                selected = true;
                return;
            case "3":
                shownQuestion.selected = 3;
                selected = true;
                return;
            case "4":
                shownQuestion.selected = 4;
                selected = true;
                return;
            case "Enter":
                await getNextQuestion();
                return;
        }
    }
</script>

<svelte:window on:keydown={onKeyDown} />

<QuestionsHandler>
    {#if base.length == 0}
        <div class="text-center text-2xl font-bold">
            Brak pytań do wyświetlenia
        </div>
    {:else}
        <button
            class="block w-full bg-gray-900 text-white py-2 px-4 rounded mb-8 hover:bg-gray-800"
            on:click={getNextQuestion}
        >
            Nastepne pytanie
        </button>

        <QuestionElement
            question={shownQuestion}
            questionNumber={shownQuestion ? shownQuestion.id + 1 : -1}
            ended={selected}
            {starEnabled}
            on:click={() => (selected = true)}
        />
    {/if}
</QuestionsHandler>
