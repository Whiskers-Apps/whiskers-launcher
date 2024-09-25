import {
	getSettings,
	LAUNCH_FIRST_KEY_OPTIONS,
	LAUNCH_SECOND_KEY_OPTIONS,
	LAUNCH_THIRD_KEY_OPTIONS,
	writeSettings,
	type Settings
} from '$lib/settings/settings';
import { invoke } from '@tauri-apps/api';
import { get, writable } from 'svelte/store';

/** General Settings UiState */
export const state = writable({
	loading: true,
	settings: {} as Settings,
	isWayland: false,
	os: '',
	firstKeyOptions: LAUNCH_FIRST_KEY_OPTIONS,
	secondKeyOptions: LAUNCH_SECOND_KEY_OPTIONS,
	thirdKeyOptions: LAUNCH_THIRD_KEY_OPTIONS,
    showShortcutWarnings: false,
});

/** Inits the current ui state with actual values */
export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.isWayland = await invoke('is_wayland');
	currentState.os = await invoke('get_os');
	currentState.loading = false;

	if (currentState.os === 'windows') {
		currentState.firstKeyOptions = [
			...LAUNCH_FIRST_KEY_OPTIONS.filter((key) => key.id !== 'super')
		];
		currentState.secondKeyOptions = [
			...LAUNCH_SECOND_KEY_OPTIONS.filter((key) => key.id !== 'super')
		];
	}

	state.update(() => currentState);
}

// ================= Intents ======================

/** Sets the first key in settings */
export function onSetFirstKey(key: CustomEvent<string>){
    let currentState = get(state);
    currentState.settings.first_key = key.detail;
    state.update(() => currentState);

    writeSettings(currentState.settings);
}

/** Sets the second key in settings */
export function onSetSecondKey(key: CustomEvent<string | null>){
    let currentState = get(state);
    currentState.settings.second_key = key.detail;
    state.update(() => currentState);

    writeSettings(currentState.settings);
}

/** Sets the third key in settings */
export function onSetThirdKey(key: CustomEvent<string>){
    let currentState = get(state);
    currentState.settings.third_key = key.detail;
    state.update(() => currentState);

    writeSettings(currentState.settings);
}

/** Sets the scaling in settings */
export function onSetScaling(scaling: CustomEvent<number>) {
	let currentState = get(state);
	currentState.settings.scaling = scaling.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

/** Sets the show recent apps in settings */
export function onSetShowRecentApps(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_recent_apps = show.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

/** Sets the autostart in settings */
export function onSetAutoStart(autoStart: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.auto_start = autoStart.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}
