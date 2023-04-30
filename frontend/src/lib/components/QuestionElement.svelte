<script lang="ts">
    import { starred } from "$lib/stores";
    import type { Question } from "$lib/types";
    import { createEventDispatcher } from "svelte";

    export let question: Question;
    export let questionNumber: number;
    export let ended: boolean;
    export let showDidntChoose: boolean = true;
    export let useKeyboard: boolean = false;

    let questionStarred: boolean = false;
    starred.subscribe((q) => {
        if (!q) return;
        questionStarred = q.includes(question.id);
    });

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

    function isCorrect(): boolean {
        return question.selected === question.correct;
    }

    function starClick() {
        if (questionStarred) {
            starred.update((q) => q.filter((id) => id !== question.id));
        } else {
            starred.update((q) => [...q, question.id]);
        }
        localStorage.setItem("starred", JSON.stringify($starred));
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
                        fill={questionStarred ? "yellow" : "none"}
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
