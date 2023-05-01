<script lang="ts">
    import QuestionsHandler from "./QuestionsHandler.svelte";
    import QuestionElement from "./QuestionElement.svelte";
    import type { Question } from "$lib/types";
    import { onMount } from "svelte";
    import { shuffleArray } from "$lib/utils";
    import { browser } from "$app/environment";

    export let base: Question[] = [];
    export let key: string = "q";

    let pool: Question[] = [];
    let question: Question;
    let wrong: number[] = [];
    let good: number[] = [];

    let done: boolean = false;
    let selected: boolean = false;

    $: percentageText = Math.round((good.length / base.length) * 10000) / 100;

    onMount(async () => {
        await init();
    });

    async function init() {
        if (!browser) return;
        wrong = JSON.parse(localStorage.getItem(`${key}Wrong`) ?? "[]");
        good = JSON.parse(localStorage.getItem(`${key}Good`) ?? "[]");
        wrong = [...new Set(wrong)];
        good = [...new Set(good)];

        // deep copy
        pool = JSON.parse(JSON.stringify(base));
        pool = pool.filter(
            (q) => !wrong.includes(q.id) && !good.includes(q.id)
        );

        setPercentage(0);
        setTimeout(() => {
            updateProgressBar();
        }, 10);
        await getNextQuestion();
    }

    async function getNextQuestion() {
        if (!selected && question) return;
        selected = false;

        if (good.length == base.length) {
            done = true;
            return;
        }

        if (pool.length == 0) {
            pool = JSON.parse(JSON.stringify(base));

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
        let percentage = good.length / base.length;
        setPercentage(percentage * 100);
    }
    function setPercentage(percentage: number) {
        document.documentElement.style.setProperty(
            "--percentage",
            `${percentage}%`
        );
    }

    function afterQuestion(e: any) {
        let correct = e.detail as boolean;
        selected = true;
        wrong = wrong.filter((x) => x != question.id);

        if (correct) {
            good.push(question.id);
            good = good;
        } else {
            wrong.push(question.id);
            wrong = wrong;
        }

        updateProgressBar();
        save();
    }

    function startOver() {
        done = false;
        good = [];
        wrong = [];

        updateProgressBar();
        getNextQuestion();
        save();
    }

    function save() {
        localStorage.setItem(`${key}Wrong`, JSON.stringify(wrong));
        localStorage.setItem(`${key}Good`, JSON.stringify(good));
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
            {good.length}/{base.length} ({percentageText}%)
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
            questionNumber={question ? question.id + 1 : -1}
            ended={selected}
            useKeyboard={true}
            on:click={afterQuestion}
        />
    {/if}
</QuestionsHandler>

<style>
    .bar {
        width: var(--percentage);
        transition: width 0.5s ease;
    }
</style>
