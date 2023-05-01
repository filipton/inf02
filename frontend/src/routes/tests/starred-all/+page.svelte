<script lang="ts">
    import WholeBaseTest from "$lib/components/WholeBaseTest.svelte";
    import type { Question } from "$lib/types";
    import { questions } from "$lib/stores";

    let base: Question[];
    questions.subscribe((x) => {
        if (base || !x) return;

        base = x.filter((x) => x.starred);
    });
</script>

{#if base && base.length > 0}
    <WholeBaseTest key="s" {base} />
{:else}
    <div class="flex flex-col items-center justify-center h-full mt-4">
        <h1 class="text-2xl font-bold">Brak pytań w bazie!</h1>
    </div>
{/if}
