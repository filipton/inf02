import { browser } from '$app/environment';
import { docs } from '$lib/stores';
import type { PageLoad, PageLoadEvent } from './$types';

export const load = (async (event: PageLoadEvent) => {
    if (!browser) return;
    const response = await event.fetch("/inf02-2022.json", {
        cache: "no-cache"
    });

    let inf02 = await response.json();
    return {
        inf02
    };
}) satisfies PageLoad;
