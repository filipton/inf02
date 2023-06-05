<script lang="ts">
    import type { Question } from "$lib/types";
    import WholeBaseTest from "$lib/components/WholeBaseTest.svelte";
    import { onMount } from "svelte";

    let base: Question[];
    onMount(async () => {
        const baseResp = await fetch("/old_base.json");
        base = await baseResp.json();
    });
</script>

{#if base && base.length > 0}
    <WholeBaseTest key="oq" {base} starEnabled={false} />
{:else}
    <div class="flex flex-col items-center justify-center h-full mt-4">
        <h1 class="text-2xl font-bold text-white">Brak pytań w bazie!</h1>
    </div>
{/if}
