import { getSettings, type Settings } from '$lib/features/settings/Settings';
import { getThemeCss } from '$lib/features/theming/Theming';
import { get, writable } from 'svelte/store';

export const state = writable({
	loading: true,
	settings: {} as Settings,
	css: ''
});

export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.css = getThemeCss(currentState.settings);
	currentState.loading = false;

	state.set(currentState);
}

export function setDialogFrameCSS(css: string){
	let currentState = get(state);
	currentState.css = css;
	state.set(currentState);
}