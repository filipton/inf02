import { browser } from '$app/environment';
import { questions } from '$lib/stores';
import type { Question } from '$lib/types';
import type { LayoutLoad, LayoutLoadEvent } from './$types';

export const load = (async (event: LayoutLoadEvent) => {
    if (!browser) return;
    const response = await event.fetch("/base.json", {
        cache: "no-cache"
    });

    let _questions: Question[] = await response.json();
    let _starred: Number[] = JSON.parse(localStorage.getItem("starred_new") || "[]");

    _questions = _questions.map((q, i) => {
        q.starred = _starred.includes(i);
        return q;
    });
    questions.set(_questions);
}) satisfies LayoutLoad;
