import { getSettings, writeSettings, type Settings } from '$lib/settings/settings';
import { get, writable } from 'svelte/store';
import { open } from '@tauri-apps/api/dialog';
import { pictureDir } from '@tauri-apps/api/path';

export const state = writable({
	loading: true,
	settings: {} as Settings
});

export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.loading = false;

	state.update(() => currentState);
}

// =================== Intents ====================

export function onSetSplitResults(split: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.split_results = split.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetShowSearchIcon(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_search_icon = show.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetShowSettingsIcon(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_settings_icon = show.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetShowPlaceholder(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_placeholder = show.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetAccentSearchBorder(set_accent: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.accent_search_border = set_accent.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetHideOnBlur(hide: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.hide_on_blur = hide.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetBorderRadius(radius: CustomEvent<number>) {
	let currentState = get(state);
	currentState.settings.border_radius = radius.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function onSetBorderWidth(width: CustomEvent<number>) {
	let currentState = get(state);
	currentState.settings.border_width = width.detail;
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export async function onSelectWallpaper() {
	const path = await open({
		defaultPath: await pictureDir(),
		filters: [
			{
				name: 'Supported Types',
				extensions: ['jpg', 'jpeg', 'png', 'gif', 'webp']
			}
		]
	});

	if (path) {
        let currentState = get(state);
        currentState.settings.wallpaper = path as string;
        state.update(() => currentState);

        writeSettings(currentState.settings);
	}
}

export function onClearWallpaper() {
    let currentState = get(state);
    currentState.settings.wallpaper = null;
    state.update(() => currentState);

    writeSettings(currentState.settings);
}