<script lang="ts">
    import { onMount } from "svelte";

    onMount(async () => {
        await loadBase();
    });

    let questions: Question[] = [];
    async function loadBase() {
        const response = await fetch("/baza.json");
        questions = await response.json();

        questions = questions.splice(0, 10);
    }

    type Question = {
        text: string;
        image: string | null;
        anwsers: string[];
        correct: number;
        selected: number; // NOT IN JSON
    };
</script>

<div class="mt-6 w-full max-w-2xl mx-auto text-white px-2">
    {#each questions as question, qi}
        <div class="question mb-12">
            <h2 class="font-bold text-xl text-center py-4 mb-2 bg-gray-900">
                {qi + 1}. {question.text}
            </h2>

            {#if question.image}
                <img
                    class="mx-auto"
                    width="100%"
                    src="/images/{question.image}"
                    alt=""
                />
            {/if}

            <div class="anwsers mt-4">
                {#each question.anwsers as anwser, i}
                    <button
                        class="block w-full bg-gray-900 text-white py-2 px-4 rounded mb-1 {question.selected ===
                        i + 1
                            ? 'bg-green-500'
                            : 'hover:bg-gray-800'}"
                        on:click={() => (question.selected = i + 1)}
                    >
                        {anwser}
                    </button>
                {/each}
            </div>
        </div>
    {/each}
</div>
