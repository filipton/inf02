<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { createEventDispatcher } from "svelte";

    export let question: Question;
    export let questionNumber: number;
    export let ended: boolean;
    export let showDidntChoose: boolean = true;
    export let useKeyboard: boolean = false;

    let dispatch = createEventDispatcher();

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

    function starClick() {
        questions.update((qs) => {
            let questionIdx = qs.findIndex((q) => q.id == question.id);
            qs[questionIdx].starred = !qs[questionIdx].starred;

            let starred = qs.filter((q) => q.starred).map((q) => q.id);
            localStorage.setItem("starred", JSON.stringify(starred));

            return qs;
        });

        question.starred = !question.starred;
        dispatch("starred", question.starred);
    }

    function isCorrect(): boolean {
        return question.selected === question.correct;
    }

    function onKeyDown(e: KeyboardEvent) {
        if (!useKeyboard || question.selected) return;

        if (e.key === "1") {
            question.selected = 1;
            dispatch("click", isCorrect());
        } else if (e.key === "2") {
            question.selected = 2;
            dispatch("click", isCorrect());
        } else if (e.key === "3") {
            question.selected = 3;
            dispatch("click", isCorrect());
        } else if (e.key === "4") {
            question.selected = 4;
            dispatch("click", isCorrect());
        }
    }
</script>

<svelte:window on:keydown={onKeyDown} />

{#if question}
    <div class="question mb-12">
        <div>
            <div class="absolute">
                <button
                    class="w-6 h-6 relative bottom-3 right-3"
                    on:click={starClick}
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill={question.starred ? "yellow" : "none"}
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="w-6 h-6"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
                        />
                    </svg>
                </button>
            </div>

            <h2
                class="font-bold text-xl text-center py-4 px-2 mb-2 bg-gray-900"
            >
                {questionNumber}. {@html question.text}
            </h2>

            {#if question.image}
                <img
                    class="mx-auto"
                    width="100%"
                    src="/images/{question.image}"
                    alt=""
                />
            {/if}

            {#if question.video}
                <video
                    class="mx-auto"
                    width="100%"
                    controls
                    src="/videos/{question.video}"
                >
                    <track kind="captions" />
                </video>
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
                        class="block w-full text-white py-2 px-4 rounded mb-1 selectable {buttonColor(
                            question,
                            i,
                            ended
                        )}"
                        on:click={() => {
                            if (ended) return;
                            question.selected = i + 1;

                            dispatch("click", isCorrect());
                        }}
                    >
                        {@html anwser}
                    </button>
                {/each}
            </div>
        </div>
    </div>
{/if}

<style>
    .selectable {
        -webkit-user-select: text;
        -moz-user-select: text;
        -ms-user-select: text;
        user-select: text;
    }
</style>
