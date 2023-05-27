<script lang="ts">
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";
    import { browser } from "$app/environment";
    import QuestionsHandler from "$lib/components/QuestionsHandler.svelte";
    import QuestionElement from "$lib/components/QuestionElement.svelte";

    let base: Question[] = [];
    let allQuestions: Question[] = [];
    let shownQuestions: Question[] = [];
    let searchQuery = "";

    questions.subscribe(async (x) => {
        if (!browser || !x) return;

        if (base.length === 0) {
            base = x;
            allQuestions = x;
            lazyLoad();
        }
    });

    function search() {
        let query = searchQuery.toLowerCase();
        if (query === "") {
            shownQuestions = base.slice(0, 50);
            return;
        }

        allQuestions = base.filter(
            (x) =>
                x.text.toLowerCase().includes(query) ||
                x.anwsers.some((a) => a.toLowerCase().includes(query)) ||
                (x.id + 1).toString() === query
        );
        allQuestions = allQuestions.sort((a, b) => {
            if ((a.id + 1).toString() === query) return -1;
            if ((b.id + 1).toString() === query) return 1;
            return 0;
        });

        shownQuestions = allQuestions.slice(0, 50);
    }

    function onScroll() {
        if (shownQuestions.length >= allQuestions.length) return;
        let fromBottom =
            document.documentElement.scrollHeight -
            window.innerHeight -
            window.scrollY;

        if (fromBottom < 10000) {
            lazyLoad();
        }
    }

    function lazyLoad(count: number = 50) {
        let questions = allQuestions.slice(0, shownQuestions.length + count);
        shownQuestions = questions;
    }
</script>

<svelte:window on:scroll={onScroll} />

<QuestionsHandler>
    <div class="mb-8">
        <input
            type="text"
            placeholder="Search (Number, Question, Anwser)"
            class="w-full px-4 py-2 text-white bg-gray-900 rounded"
            bind:value={searchQuery}
            on:input={search}
        />
    </div>

    {#each shownQuestions as question}
        <QuestionElement
            {question}
            questionNumber={question.id + 1}
            ended={true}
            showDidntChoose={false}
            on:starred={() => {
                question.starred = !question.starred;
            }}
        />
    {/each}
</QuestionsHandler>
