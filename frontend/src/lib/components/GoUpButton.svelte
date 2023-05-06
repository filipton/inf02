<script lang="ts">
    let showButton = false;

    function handleScroll() {
        showButton = window.scrollY > 100;
    }
</script>

<svelte:window on:scroll={handleScroll} />

<div class="absolute up-button {showButton ? 'show' : 'hide'}">
    <div class="fixed bottom-0 right-0 m-4">
        <button
            class="border-red-100 bg-black hover:bg-gray-800 text-white font-bold py-2 px-2 rounded"
            on:click={() => {
                window.scrollTo({ top: 0, behavior: "smooth" });
            }}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M4.5 12.75l7.5-7.5 7.5 7.5m-15 6l7.5-7.5 7.5 7.5"
                />
            </svg>
        </button>
    </div>
</div>

<style>
    .up-button {
        transition: opacity 0.2s ease-in-out;
    }

    .show {
        opacity: 1;
        z-index: 100;
    }

    .hide {
        opacity: 0;
        z-index: -100;
    }
</style>
