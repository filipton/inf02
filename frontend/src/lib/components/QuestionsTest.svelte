<script lang="ts">
    import type { Question, SavedState } from "$lib/types";
    import QuestionElement from "$lib/components/QuestionElement.svelte";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import { shuffleArray, msToTime } from "$lib/utils";
    import { onMount } from "svelte";

    export let key: string;
    export let base: Question[];
    export let testSize: number = 40;

    export let starEnabled: boolean = true;

    let questionPool: Question[];
    let shownQuestions: Question[] = [];
    let percentage = 0;
    let startedAt = -1;
    let ended = false;

    let currentQuestionKB = -1;
    let elements: HTMLElement[] = [];

    onMount(async () => {
        await getQuestionsSet();
    });

    async function getQuestionsSet() {
        if (localStorage.getItem(`${key}${testSize}State`)) {
            let state: SavedState = JSON.parse(
                localStorage.getItem(`${key}${testSize}State`) || ""
            );
            shownQuestions = state.questions;
            startedAt = state.startedAt;

            return;
        }

        if (!questionPool || questionPool.length < testSize) {
            questionPool = JSON.parse(JSON.stringify(base));
        }

        shuffleArray(questionPool);
        shownQuestions = questionPool.slice(0, testSize);
        questionPool = questionPool.slice(testSize);

        await scrambleAnwsers();
        startedAt = Date.now();
    }

    async function scrambleAnwsers() {
        for (const question of shownQuestions) {
            let old = question.anwsers[question.correct - 1];
            shuffleArray(question.anwsers);

            question.correct = question.anwsers.findIndex((x) => x === old) + 1;
        }
    }

    function percentageColor() {
        if (percentage < 0.5) return "bg-red-600";
        return "bg-lime-600";
    }

    async function bottomButton() {
        localStorage.removeItem(`${key}${testSize}State`);
        if (ended) {
            await getQuestionsSet();
            ended = false;
            window.scrollTo({ top: 0 });

            return;
        }

        ended = true;
        percentage =
            shownQuestions.filter((x) => x.selected == x.correct).length /
            shownQuestions.length;

        window.scrollTo({ top: 0 });
    }

    function saveState() {
        let state: SavedState = {
            questions: shownQuestions,
            startedAt,
        };
        localStorage.setItem(`${key}${testSize}State`, JSON.stringify(state));
    }

    function onKeyDown(e: KeyboardEvent) {
        if (e.key === "Enter") {
            currentQuestionKB++;
            if (currentQuestionKB >= elements.length) {
                bottomButton();
                currentQuestionKB = -1;
            }

            elements[currentQuestionKB].scrollIntoView();
        } else if (e.key == "Backspace") {
            currentQuestionKB--;
            if (currentQuestionKB < 0) {
                currentQuestionKB = 0;
            }

            elements[currentQuestionKB].scrollIntoView();
        } else if (e.key == "1") {
            shownQuestions[currentQuestionKB].selected = 1;
        } else if (e.key == "2") {
            shownQuestions[currentQuestionKB].selected = 2;
        } else if (e.key == "3") {
            shownQuestions[currentQuestionKB].selected = 3;
        } else if (e.key == "4") {
            shownQuestions[currentQuestionKB].selected = 4;
        }
    }
</script>

<svelte:window on:keydown={onKeyDown} />

<QuestionsHandler>
    {#if base.length < testSize}
        <span
            class="block w-full text-white py-2 px-4 my-4 rounded center text-center bg-red-600"
        >
            Nie ma wystarczającej ilości pytań do wygenerowania testu!
        </span>
    {:else}
        {#if ended}
            <span
                class="block w-full text-white py-2 px-4 my-4 rounded center text-center {percentageColor()}"
            >
                Zdobyłeś {percentage * 100}%! [{percentage *
                    testSize}/{testSize}]

                <br />
                W czasie: {msToTime(Date.now() - startedAt)}
            </span>
        {/if}

        {#each shownQuestions as question, i}
            <div bind:this={elements[i]}>
                <QuestionElement
                    {question}
                    questionNumber={i + 1}
                    {ended}
                    {starEnabled}
                    on:click={saveState}
                />
            </div>
        {/each}

        <button
            class="block w-full bg-gray-900 text-white py-2 px-4 rounded my-2 hover:bg-gray-800"
            on:click={bottomButton}
        >
            {ended ? "Nastepny test" : "Zakoncz test"}
        </button>
    {/if}
</QuestionsHandler>
