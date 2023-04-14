import { questions } from '$lib/stores';
import type { Question } from '$lib/types';
import type { LayoutLoad, LayoutLoadEvent } from './$types';

export const load = (async (event: LayoutLoadEvent) => {
    const response = await event.fetch("/base.json", {
        cache: "no-cache"
    });
    let _questions: Question[] = await response.json();

    questions.set(_questions);
}) satisfies LayoutLoad;
