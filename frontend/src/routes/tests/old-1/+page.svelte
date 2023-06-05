<script lang="ts">
    import type { Question } from "$lib/types";
    import OneQuestionTest from "$lib/components/OneQuestionTest.svelte";
    import { onMount } from "svelte";

    let base: Question[];
    onMount(async () => {
        const baseResp = await fetch("/old_base.json");
        base = await baseResp.json();
    });
</script>

{#if base}
    <OneQuestionTest {base} starEnabled={false} />
{/if}
