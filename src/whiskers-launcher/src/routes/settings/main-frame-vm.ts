import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
import { get, writable } from 'svelte/store';

export const state = writable({
    loading: true,
	css: ''
});

/** Inits the frame with the css from settings */
export async function init() {
	let currentState = get(state);
	let settings = await getSettings();

	currentState.css = getThemeCss(settings);
    currentState.loading = false;
	state.update(() => currentState);
}

export function setFrameCSS(css: string){
	let currentState = get(state);
    currentState.css = css;
    state.update(() => currentState);
}