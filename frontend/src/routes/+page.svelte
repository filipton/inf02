<script lang="ts">
    import { onMount } from "svelte";

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

        window.scrollTo({ top: 0, behavior: "smooth" });
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
    {#if ended}
        <span
            class="block w-full text-white py-2 px-4 my-4 rounded center text-center {percentageColor()}"
        >
            Zdobyłeś {percentage * 100}%! [{percentage * 40}/40]
        </span>
    {/if}

    {#each questions as question}
        <div class="question mb-12">
            <div>
                <h2
                    class="font-bold text-xl text-center py-4 px-2 mb-2 bg-gray-900"
                >
                    {questions.findIndex((x) => x == question) + 1}. {question.text}
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
                {#if question.selected === undefined && ended}
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
    {/each}

    {#if ended}
        <span
            class="block w-full text-white py-2 px-4 rounded my-4 center text-center {percentageColor()}"
        >
            Zdobyłeś {percentage * 100}%! [{percentage * 40}/40]
        </span>
    {/if}

    <button
        class="block w-full bg-gray-900 text-white py-2 px-4 rounded my-2 hover:bg-gray-800"
        on:click={end}
    >
        {ended ? "Nastepny test" : "Zakoncz test"}
    </button>
</div>
