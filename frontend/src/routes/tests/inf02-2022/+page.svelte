<script lang="ts">
    import WholeBaseTest from "$lib/components/WholeBaseTest.svelte";
    import type { Question } from "$lib/types";
    import { page } from "$app/stores";

    let base: Question[] = $page.data.inf02;
</script>

{#if base && base.length > 0}
    <WholeBaseTest key="i0222" {base} />
{:else}
    <div class="flex flex-col items-center justify-center h-full mt-4">
        <h1 class="text-2xl font-bold text-white">Brak pytań w bazie!</h1>
    </div>
{/if}
