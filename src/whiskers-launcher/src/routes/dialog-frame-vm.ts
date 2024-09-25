import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
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

	state.update(() => currentState);
}

export function setDialogFrameCSS(css: string){
	let currentState = get(state);
	currentState.css = css;
	state.update(() => currentState);
}