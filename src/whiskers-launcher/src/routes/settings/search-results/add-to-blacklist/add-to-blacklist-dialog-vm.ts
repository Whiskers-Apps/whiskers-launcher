import { get, writable } from 'svelte/store';
import { type App } from '$lib/settings/settings';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';

export const state = writable({
	loading: true,
	whiteListedApps: [] as App[],
	selectedApps: [] as string[]
});

export async function init() {
	let currentState = get(state);
	currentState.whiteListedApps = await invoke('get_whitelisted_apps');
	currentState.loading = false;
	state.set(currentState);
}

export function onAppClick(app: App) {
	let currentState = get(state);

	if (currentState.selectedApps.includes(app.id)) {
		currentState.selectedApps = currentState.selectedApps.filter((appId) => appId !== app.id);
	} else {
		currentState.selectedApps.push(app.id);
	}

	state.set(currentState);
}

export async function onAddAppsToBlacklist() {
	let currentState = get(state);

	for (const id in currentState.selectedApps) {
		await invoke('add_to_blacklist', { id: currentState.selectedApps[id] });
	}

	await emit('refresh-blacklist');

	appWindow.close();
}

export function getHighlightClass(id: string): string {
	let currentState = get(state);

	if (currentState.selectedApps.includes(id)) {
		return 'bg-secondary border border-accent';
	}

	return 'border border-transparent';
}
