import type { Actions } from '@sveltejs/kit';
import type { ServerLoad } from '@sveltejs/kit';
export const load: ServerLoad = async ({ locals }) => {
	return {};
};
export const actions: Actions = {};