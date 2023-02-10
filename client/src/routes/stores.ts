import { writable, derived } from 'svelte/store';
import type { MeQuery } from '../codegen';

export type IUser = MeQuery['me'];

export const userData = writable<IUser>(null);

export const user = derived(userData, ($userData) => {
	if ($userData) {
		return { id: $userData.id, email: $userData.email, role: $userData.role };
	}
	return null;
});

export const pageMargin = writable(0);
export const facilityId = writable<string>('00000000000000000000000000000000');
