import { writable, derived } from 'svelte/store';
import type { MeQuery } from '../codegen';

export type IUser = MeQuery['me'];

export const userData = writable<IUser>(null);

export const user = derived(userData, ($userData) => {
	if ($userData) {
		return { id: $userData.id, email: $userData.email };
	}
	return null;
});

export const pageMargin = writable(0);
