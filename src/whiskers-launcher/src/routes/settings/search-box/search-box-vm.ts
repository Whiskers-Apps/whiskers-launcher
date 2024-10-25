import { get, writable } from 'svelte/store';
import { open } from '@tauri-apps/api/dialog';
import { pictureDir } from '@tauri-apps/api/path';
import { getSettings, writeSettings, type Settings } from '$lib/features/settings/Settings';

export const state = writable({
	loading: true,
	settings: {} as Settings
});

export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.loading = false;

	state.set(currentState);
}

// =================== Intents ====================

export function onSetShowSearchIcon(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_search_icon = show.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetShowSettingsIcon(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_settings_icon = show.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetShowPlaceholder(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_placeholder = show.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetHideOnBlur(hide: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.hide_on_blur = hide.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetBorderRadius(radius: CustomEvent<number>) {
	let currentState = get(state);
	currentState.settings.border_radius = radius.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetBorderWidth(radius: CustomEvent<number>){
	let currentState = get(state);
	currentState.settings.border_width = radius.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
};

export function onSetAccentBorder(setAccent: CustomEvent<boolean>){
	let currentState = get(state);
	currentState.settings.accent_border = setAccent.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
};

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
        state.set(currentState);

        writeSettings(currentState.settings);
	}
}

export function onClearWallpaper() {
    let currentState = get(state);
    currentState.settings.wallpaper = null;
    state.set(currentState);

    writeSettings(currentState.settings);
}
