import { writable, type Writable } from 'svelte/store';
import type { Question } from './types';

export const questions: Writable<Question[]> = writable();
export const starred: Writable<Number[]> = writable();
