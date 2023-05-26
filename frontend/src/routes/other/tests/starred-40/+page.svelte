<script lang="ts">
    import type { Question, SavedState } from "$lib/types";
    import QuestionElement from "$lib/components/QuestionElement.svelte";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import { shuffleArray, msToTime } from "$lib/utils";

    let questionPool: Question[];
    let shownQuestions: Question[] = [];
    let percentage = 0;
    let startedAt = -1;
    let ended = false;

    questions.subscribe(async (x) => {
        if (!browser || questionPool) return;

        questionPool = JSON.parse(JSON.stringify(x.filter((q) => q.starred)));
        await getQuestionsSet();
    });

    async function getQuestionsSet() {
        if (localStorage.getItem("q40State")) {
            let state: SavedState = JSON.parse(
                localStorage.getItem("q40State") || ""
            );
            shownQuestions = state.questions;
            startedAt = state.startedAt;

            return;
        }

        if (!questionPool || questionPool.length < 40) {
            questionPool = JSON.parse(
                JSON.stringify($questions.filter((q) => q.starred))
            );
        }

        shuffleArray(questionPool);
        shownQuestions = questionPool.slice(0, 40);
        questionPool = questionPool.slice(40);

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
        localStorage.removeItem("s40State");
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
        localStorage.setItem("q40State", JSON.stringify(state));
    }
</script>

<QuestionsHandler>
    {#if shownQuestions.length < 40}
        <div class="text-center">
            <h1 class="text-2xl font-bold mb-4">
                Brak pytań z gwiazdka (min 40)
            </h1>
        </div>
    {:else}
        {#if ended}
            <span
                class="block w-full text-white py-2 px-4 my-4 rounded center text-center {percentageColor()}"
            >
                Zdobyłeś {percentage * 100}%! [{percentage * 40}/40]

                <br />
                W czasie: {msToTime(Date.now() - startedAt)}
            </span>
        {/if}

        {#each shownQuestions as question, i}
            <QuestionElement
                {question}
                questionNumber={i + 1}
                {ended}
                on:click={saveState}
            />
        {/each}

        <button
            class="block w-full bg-gray-900 text-white py-2 px-4 rounded my-2 hover:bg-gray-800"
            on:click={bottomButton}
        >
            {ended ? "Nastepny test" : "Zakoncz test"}
        </button>
    {/if}
</QuestionsHandler>
