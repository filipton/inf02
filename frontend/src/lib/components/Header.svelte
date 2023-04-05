<script lang="ts">
    import { onMount } from "svelte";
    let current: HTMLElement;

    onMount(() => {
        let url = window.location.href;
        let clicked = document.querySelector(
            `a[href="/${url.split("/").slice(3)[0]}"]`
        ) as HTMLElement;

        calcCurrent(clicked);
    });

    function navClick() {
        //@ts-ignore
        let clicked = this as HTMLElement;
        calcCurrent(clicked);
    }

    function calcCurrent(clicked: HTMLElement) {
        let clickedWidth = clicked.offsetWidth + 4;
        let clickedLeft = clicked.offsetLeft - current.offsetLeft - 8 - 2;

        document.documentElement.style.setProperty(
            "--current-width",
            `${clickedWidth}px`
        );
        document.documentElement.style.setProperty(
            "--current-left",
            `${clickedLeft}px`
        );
    }
</script>

<div
    bind:this={current}
    class="bg-transparent text-white max-w-2xl w-full mx-auto px-2 border-b-2 border-white current"
>
    <div class="flex justify-between">
        <div class="flex">
            <a on:click={navClick} href="/tests" class="text-white p-2">TESTY</a
            >
        </div>
        <div class="flex">
            <a
                on:click={navClick}
                href="/"
                class="text-white font-bold text-center w-full p-2"
                >inf02.filipton.space</a
            >
        </div>
        <div class="flex">
            <a on:click={navClick} href="/base" class="text-white p-2">BAZA</a>
        </div>
    </div>
</div>

<style>
    .current::after {
        position: relative;
        content: "";
        display: block;
        width: var(--current-width);
        left: var(--current-left);
        height: 2px;
        background: gray;
        bottom: -2px;
        transition: all 0.3s ease;

        /*
        opacity: 0;
        animation: show 1s ease forwards;
        */
    }

    /*
    @keyframes show {
        0% {
            opacity: 0;
        }
        100% {
            opacity: 1;
        }
    }
    */
</style>
