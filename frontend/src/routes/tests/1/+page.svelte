<script lang="ts">
    import type { Question } from "$lib/types";
    import QuestionElement from "$lib/components/QuestionElement.svelte";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import { shuffleArray } from "$lib/utils";

    let allQuestions: Question[] = [];
    let questionsPool: Question[] = [];
    let shownQuestion: Question;
    let shownQuestionId: number;

    let selected = false;

    questions.subscribe(async (x) => {
        if (!browser) return;

        allQuestions = x;
        questionsPool = JSON.parse(JSON.stringify(x));
        await getNextQuestion();
    });

    async function getNextQuestion() {
        selected = false;
        if (questionsPool.length == 0) {
            questionsPool = JSON.parse(JSON.stringify(allQuestions));
        }

        let randomId = Math.floor(Math.random() * questionsPool.length);
        shownQuestion = questionsPool[randomId];
        questionsPool = removeAt(questionsPool, randomId);

        shownQuestionId = allQuestions.findIndex(
            (x) =>
                x.text == shownQuestion.text &&
                x.correct == shownQuestion.correct &&
                x.image == shownQuestion.image
        );
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
        if (e.key === "ArrowRight") {
            await getNextQuestion();
            console.log(questionsPool.length);
        }
    }
</script>

<svelte:window on:keydown={onKeyDown} />

<QuestionsHandler>
    <button
        class="block w-full bg-gray-900 text-white py-2 px-4 rounded mb-8 hover:bg-gray-800"
        on:click={getNextQuestion}
    >
        Nastepne pytanie
    </button>

    <QuestionElement
        question={shownQuestion}
        questionNumber={shownQuestionId}
        ended={selected}
        on:click={() => (selected = true)}
    />
</QuestionsHandler>
