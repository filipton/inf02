<script lang="ts">
    import { onMount } from "svelte";
    import type { Question } from "$lib/types";
    import QuestionElement from "$lib/components/QuestionElement.svelte";
    import { text } from "svelte/internal";

    let allQuestions: Question[] = [];
    let questions: Question[] = [];
    let percentage = 0;
    let ended = false;

    onMount(async () => {
        await loadBase();
        await getAnwsers();
    });

    async function loadBase() {
        const response = await fetch("/baza.json");
        allQuestions = await response.json();
    }

    async function getAnwsers() {
        // deep copy
        questions = JSON.parse(JSON.stringify(allQuestions));
        shuffleArray(questions);
        questions = questions.slice(0, 40);

        await scrambleAnwsers();
    }

    async function scrambleAnwsers() {
        for (const question of questions) {
            let old = question.anwsers[question.correct - 1];
            shuffleArray(question.anwsers);

            question.correct = question.anwsers.findIndex((x) => x === old) + 1;
        }
    }

    function percentageColor() {
        if (percentage < 0.5) return "bg-red-600";
        return "bg-lime-600";
    }

    function shuffleArray(array: any[]) {
        for (var i = array.length - 1; i > 0; i--) {
            var j = Math.floor(Math.random() * (i + 1));
            var temp = array[i];
            array[i] = array[j];
            array[j] = temp;
        }
    }

    async function end() {
        if (ended) {
            await getAnwsers();
            ended = false;
            window.scrollTo({ top: 0, behavior: "smooth" });

            return;
        }

        ended = true;
        percentage =
            questions.filter((x) => x.selected == x.correct).length /
            questions.length;

        window.scrollTo({ top: 0 });
    }
</script>

<div class="mt-6 w-full max-w-2xl mx-auto text-white px-2">
    {#if ended}
        <span
            class="block w-full text-white py-2 px-4 my-4 rounded center text-center {percentageColor()}"
        >
            Zdobyłeś {percentage * 100}%! [{percentage * 40}/40]
        </span>
    {/if}

    {#each questions as question, i}
        <QuestionElement
            {question}
            questionNumber={i + 1}
            {ended}
            debugString={allQuestions
                .findIndex(
                    (x) => x.text == question.text && x.image == question.image
                )
                .toString()}
        />
    {/each}

    <button
        class="block w-full bg-gray-900 text-white py-2 px-4 rounded my-2 hover:bg-gray-800"
        on:click={end}
    >
        {ended ? "Nastepny test" : "Zakoncz test"}
    </button>
</div>
