import { writable, type Writable } from 'svelte/store';
import type { DocsEntry, Question } from './types';

export const questions: Writable<Question[]> = writable();
export const docs: Writable<DocsEntry[]> = writable();
