import {
	getSettings,
	getThemeCss,
	writeSettings,
	type Settings,
	type Theme
} from '$lib/settings/settings';
import { WebviewWindow } from '@tauri-apps/api/window';
import { get, writable } from 'svelte/store';
import { WindowSizes } from '../../../utils';
import { listen } from '@tauri-apps/api/event';
import { open, save } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';
import { setFrameCSS } from '../main-frame-vm';

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

async function refreshSettings() {
	let currentState = get(state);
	currentState.settings = await getSettings();

	setFrameCSS(getThemeCss(currentState.settings));

	state.update(() => currentState);
}

export async function onOpenStore() {
	new WebviewWindow('themes-store', {
		url: 'settings/theming/themes-store',
		title: 'Themes Store',
		height: WindowSizes.Store.height,
		width: WindowSizes.Store.width,
		resizable: false,
		maximizable: false
	});

	const _ = await listen('refresh-theme', async () => {
		refreshSettings();
	});
}

export async function onImportTheme() {
	const path = await open({
		defaultPath: await downloadDir(),
		filters: [{ name: 'Whiskers Theme', extensions: ['wltheme'] }]
	});

	if (path) {
		invoke('get_theme_from_file', { path: path }).then(async (theme) => {
			let currentState = get(state);
			currentState.settings.theme = theme as Theme;
			state.update(() => currentState);

			writeSettings(currentState.settings);
			setFrameCSS(getThemeCss(currentState.settings));
		});
	}
}

export async function onExportTheme() {
	const path = await save({
		defaultPath: `${await downloadDir()}/theme.wltheme`,
		filters: [{ name: 'Whiskers Theme', extensions: ['wltheme'] }]
	});

	if (path) {
		invoke('export_theme', { path: path });
	}
}

export function onSetBackgroundColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.background = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetSecondaryColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.secondary = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetTertiaryColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.tertiary = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetAccentColor(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
	let currentState = get(state);
	currentState.settings.theme.accent = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetWarningColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.warning = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetDangerColor(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
	let currentState = get(state);
	currentState.settings.theme.danger = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetOnAccentColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.on_accent = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetOnDangerColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.on_danger = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetTextColor(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
	let currentState = get(state);
	currentState.settings.theme.text = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}

export function onSetSubTextColor(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.settings.theme.sub_text = event.currentTarget.value;

	state.update(() => currentState);
	setFrameCSS(getThemeCss(currentState.settings));

	writeSettings(currentState.settings);
}
