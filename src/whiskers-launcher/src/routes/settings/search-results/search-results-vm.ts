import { getSettings, writeSettings, type Settings, type App } from '$lib/settings/settings';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window';
import { get, writable } from 'svelte/store';

export const state = writable({
	loading: true,
	settings: {} as Settings,
	blacklist_apps: [] as App[]
});

export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.blacklist_apps = await invoke('get_blacklisted_apps');
	currentState.loading = false;

	state.set(currentState);
}

// =================== Intents ====================

export function onSetHighlightSelectedBackground(highlight: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.highlight_selected_background = highlight.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetShowAltHint(show: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.settings.show_alt_hint = show.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export function onSetResultsCount(count: CustomEvent<number>) {
	let currentState = get(state);
	currentState.settings.results_count = count.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export async function onOpenAddToBlacklistDialog() {
	new WebviewWindow('add-to-blacklist', {
		url: 'settings/search-results/add-to-blacklist',
		title: 'Add To Blacklist',
		resizable: false,
		width: 800,
		height: 700,
		center: true
	});

	const unlisten = await listen('refresh-blacklist', async (_) => {
		let currentState = get(state);
		currentState.blacklist_apps = await invoke('get_blacklisted_apps');
        state.set(currentState);
        
		unlisten();
	});
}

export async function onOpenRemoveFromBlacklistDialog(id: string) {
	new WebviewWindow('remove-from-blacklist', {
		url: `settings/search-results/remove-from-blacklist/?id=${encodeURIComponent(id)}`,
		title: 'Remove from Blacklist',
		resizable: false,
		width: 800,
		height: 200,
		center: true
	});

	const unlisten = await listen('refresh-blacklist', async (_) => {
		let currentState = get(state);
        currentState.blacklist_apps = await invoke('get_blacklisted_apps');
        state.set(currentState);

		unlisten();
	});
}
