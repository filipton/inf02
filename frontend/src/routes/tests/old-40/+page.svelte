<script lang="ts">
    import QuestionsTest from "$lib/components/QuestionsTest.svelte";
    import type { Question } from "$lib/types";
    import { onMount } from "svelte";

    let base: Question[];
    onMount(async () => {
        const baseResp = await fetch("/old_base.json");
        base = await baseResp.json();
    });
</script>

{#if base}
    <QuestionsTest key="oq" {base} testSize={40} starEnabled={false} />
{/if}
