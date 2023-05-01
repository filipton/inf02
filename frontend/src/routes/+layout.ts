import { browser } from '$app/environment';
import { questions, starred } from '$lib/stores';
import type { Question } from '$lib/types';
import type { LayoutLoad, LayoutLoadEvent } from './$types';

export const load = (async (event: LayoutLoadEvent) => {
    if (browser) {
        let _starred: Number[] = JSON.parse(localStorage.getItem("starred") || "[]");
        starred.set(_starred);
    }

    const response = await event.fetch("/base.json", {
        cache: "no-cache"
    });
    let _questions: Question[] = await response.json();
    questions.set(_questions);
}) satisfies LayoutLoad;
