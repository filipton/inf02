<script lang="ts">
    import type { Question } from "$lib/types";

    export let question: Question;
    export let questionNumber: number;
    export let ended: boolean;
    export let showDidntChoose: boolean = true;

    function buttonColor(
        q: Question,
        index: number,
        stateShowAnwsers: boolean
    ) {
        if (!stateShowAnwsers && q.selected == index + 1) {
            return "bg-gray-700";
        }

        if (q.selected == index + 1) {
            if (q.selected === q.correct) return "bg-lime-600";
            if (q.selected !== q.correct) return "bg-red-600";
        }

        if (stateShowAnwsers && q.correct == index + 1) {
            return "bg-lime-600";
        }

        return "bg-gray-900";
    }
</script>

<div class="question mb-12">
    <div>
        <h2 class="font-bold text-xl text-center py-4 px-2 mb-2 bg-gray-900">
            {questionNumber}. {question.text}
        </h2>

        {#if question.image}
            <img
                class="mx-auto"
                width="100%"
                src="/images/{question.image}"
                alt=""
            />
        {/if}
    </div>

    <div>
        {#if question.selected === undefined && ended && showDidntChoose}
            <span
                class="block w-full bg-red-600 text-white py-2 px-4 rounded center"
            >
                Nie zaznaczyles odpowiedzi!
            </span>
        {/if}

        <div class="anwsers mt-4">
            {#each question.anwsers as anwser, i}
                <button
                    class="block w-full text-white py-2 px-4 rounded mb-1 {buttonColor(
                        question,
                        i,
                        ended
                    )}"
                    on:click={() => {
                        if (ended) return;
                        question.selected = i + 1;
                    }}
                >
                    {anwser}
                </button>
            {/each}
        </div>
    </div>
</div>
