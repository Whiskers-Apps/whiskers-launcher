import type { SelectValue } from '$lib/components/classes';
import { getSettings, LAUNCH_FIRST_KEY_OPTIONS, LAUNCH_SECOND_KEY_OPTIONS, LAUNCH_THIRD_KEY_OPTIONS, writeSettings, type Settings } from '$lib/features/settings/Settings';
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
	currentState.isWayland = await invoke('run_on_wayland');
	currentState.os = await invoke('run_get_os');
	currentState.loading = false;

	if (currentState.os === 'windows') {
		currentState.firstKeyOptions = [
			...LAUNCH_FIRST_KEY_OPTIONS.filter((key) => key.id !== 'super')
		];
		currentState.secondKeyOptions = [
			...LAUNCH_SECOND_KEY_OPTIONS.filter((key) => key.id !== 'super')
		];
	}

	state.set(currentState);
}

// ================= Intents ======================

/** Sets the first key in settings */
export function onSetFirstKey(key: CustomEvent<SelectValue>){
    let currentState = get(state);
    currentState.settings.first_key = key.detail.id;
    state.set(currentState);

    writeSettings(currentState.settings);
}

/** Sets the second key in settings */
export function onSetSecondKey(key: CustomEvent<SelectValue>){
    let currentState = get(state);
    currentState.settings.second_key = key.detail.id === "-" ? null : key.detail.id;
    state.set(currentState);

    writeSettings(currentState.settings);
}

/** Sets the third key in settings */
export function onSetThirdKey(key: CustomEvent<SelectValue>){
    let currentState = get(state);
    currentState.settings.third_key = key.detail.id;
    state.set(currentState);

    writeSettings(currentState.settings);
}

/** Sets the show recent apps in settings */
export function onSetShowRecentApps(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_recent_apps = show.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

/** Sets the autostart in settings */
export function onSetAutoStart(autoStart: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.auto_start = autoStart.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}
