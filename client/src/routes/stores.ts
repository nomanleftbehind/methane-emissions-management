import { writable } from 'svelte/store';
import type { MeQuery } from '../codegen';

export type IUser = Omit<NonNullable<MeQuery['me']>, '__typename'>;

export const user = writable<IUser | null>(null);