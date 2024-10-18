import { getSettings } from '$lib/features/settings/Settings';
import { getThemeCss } from '$lib/features/theming/Theming';
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
	state.set(currentState);
}

export function setFrameCSS(css: string){
	let currentState = get(state);
    currentState.css = css;
    state.set(currentState);
}