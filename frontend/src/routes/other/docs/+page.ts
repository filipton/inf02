import { browser } from '$app/environment';
import { docs } from '$lib/stores';
import type { DocsEntry } from '$lib/types';
import type { PageLoad, PageLoadEvent } from './$types';

export const load = (async (event: PageLoadEvent) => {
    if (!browser) return;
    const response = await event.fetch("/docs.json", {
        cache: "no-cache"
    });

    let _docs: DocsEntry[] = await response.json();
    docs.set(_docs);
}) satisfies PageLoad;
