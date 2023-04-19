<script lang="ts">
    import QuestionsHandler from "./QuestionsHandler.svelte";
    import QuestionElement from "./QuestionElement.svelte";
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import { shuffleArray } from "$lib/utils";

    let done: boolean = false;

    let wrong: number[] = [];
    let good: number[] = [];

    let pool: Question[] = [];
    let question: Question;
    let selected = false;

    $: percentageText =
        Math.round((good.length / $questions.length) * 10000) / 100;

    onMount(() => {});

    questions.subscribe(async (qs) => {
        if (!browser) return;
        wrong = JSON.parse(localStorage.getItem("qWrong") ?? "[]");
        good = JSON.parse(localStorage.getItem("qGood") ?? "[]");

        wrong = [...new Set(wrong)];
        good = [...new Set(good)];
        localStorage.setItem("qWrong", JSON.stringify(wrong));
        localStorage.setItem("qGood", JSON.stringify(good));

        // deep copy
        pool = JSON.parse(JSON.stringify(qs));
        pool = pool.filter((_, i) => !wrong.includes(i) && !good.includes(i));

        updateProgressBar();
        await getNextQuestion();
    });

    async function getNextQuestion() {
        if (!selected && question) return;
        selected = false;

        if (good.length == $questions.length) {
            done = true;
            return;
        }

        if (pool.length == 0) {
            pool = JSON.parse(JSON.stringify($questions));

            let tmpPool = pool.filter(
                (_, i) => !wrong.includes(i) && !good.includes(i)
            );

            if (tmpPool.length == 0) {
                pool = pool.filter((_, i) => !good.includes(i));
            } else {
                pool = tmpPool;
            }
        }

        question = pool[0];
        pool = pool.slice(1);

        await scrambleAnwsers();
    }

    async function scrambleAnwsers() {
        let old = question.anwsers[question.correct - 1];
        shuffleArray(question.anwsers);

        question.correct = question.anwsers.findIndex((x) => x == old) + 1;
    }

    function updateProgressBar() {
        let percentage = good.length / $questions.length;

        document.documentElement.style.setProperty(
            "--percentage",
            `${percentage * 100}%`
        );
    }

    function afterQuestion(e: any) {
        let correct = e.detail as boolean;
        selected = true;

        if (correct) {
            good.push(question.id);
            good = good;

            if (wrong.includes(question.id)) {
                wrong = wrong.filter((x) => x != question.id);
            }
        } else {
            wrong = wrong.filter((x) => x != question.id);
            wrong.push(question.id);
            wrong = wrong;
        }

        updateProgressBar();
        localStorage.setItem("qWrong", JSON.stringify(wrong));
        localStorage.setItem("qGood", JSON.stringify(good));
    }

    function startOver() {
        done = false;
        good = [];
        wrong = [];

        localStorage.setItem("qWrong", JSON.stringify(wrong));
        localStorage.setItem("qGood", JSON.stringify(good));

        updateProgressBar();
        getNextQuestion();
    }

    function onKeyDown(e: KeyboardEvent) {
        if (e.key == "Enter") {
            getNextQuestion();
        }
    }
</script>

<svelte:window on:keydown={onKeyDown} />

<QuestionsHandler>
    <div class="w-full h-6 rounded-lg border border-white relative">
        <div
            class="z-50 absolute w-full h-full text-center font-bold drop-shadow-[0_1.2px_1.2px_rgba(0,0,0,0.8)]"
        >
            {good.length}/{$questions.length} ({percentageText}%)
        </div>

        <div class="absolute h-full top-0 bg-lime-600 rounded-lg bar" />
    </div>

    {#if done}
        <button
            class="block w-full bg-gray-900 text-white py-2 px-4 rounded hover:bg-gray-800 my-16"
            on:click={startOver}
        >
            Zacznij od nowa
        </button>
    {:else}
        <button
            class="block w-full bg-gray-900 text-white py-2 px-4 rounded mb-8 mt-2 hover:bg-gray-800"
            on:click={getNextQuestion}
        >
            Nastepne pytanie
        </button>

        <QuestionElement
            {question}
            questionNumber={question ? question.id : -1}
            ended={selected}
            useKeyboard={true}
            on:click={afterQuestion}
        />
    {/if}
</QuestionsHandler>

<style>
    ::root {
        --percentage: 0%;
    }

    .bar {
        width: var(--percentage);
        transition: width 0.5s ease;
    }
</style>
