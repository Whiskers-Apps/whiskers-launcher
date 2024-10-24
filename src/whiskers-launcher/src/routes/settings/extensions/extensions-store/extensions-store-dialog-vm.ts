import type { ExtensionManifest } from '$lib/features/extensions/Extensions';
import type { ExtensionStoreItem } from '$lib/features/settings/Settings';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import axios from 'axios';
import { get, writable } from 'svelte/store';

export const state = writable({
	loading: true,
	searchText: '',
	store: [] as ExtensionStoreItem[],
	filteredStore: [] as ExtensionStoreItem[],
	displayedStore: [] as ExtensionStoreItem[],
	page: 0,
	installedExtensions: [] as string[],
	installingExtension: false
});

export async function init() {
	let currentState = get(state);

	let extensions: ExtensionManifest[] = await invoke('run_get_extensions');
	extensions.forEach((extension) => {
		currentState.installedExtensions.push(extension.id);
	});

	let os: string = await invoke('run_get_os');

	currentState.store = await invoke('run_get_extensions_store');
	currentState.store = currentState.store.filter((extension) => extension?.os?.includes(os));
	currentState.filteredStore = currentState.store;
	currentState.displayedStore = currentState.store.slice(0, 12);
	currentState.loading = currentState.store.length === 0;

	state.set(currentState);

	axios
		.get(
			'https://raw.githubusercontent.com/Whiskers-Apps/whiskers-launcher-extensions/main/extensions.json'
		)
		.then((response) => {
			currentState.store = response.data;
			currentState.filteredStore = currentState.store;
			currentState.displayedStore = currentState.store.slice(0, 12);
			currentState.loading = false;

			invoke('run_write_extensions_store', { store: currentState.store });

			state.set(currentState);
		})
		.catch((error) => console.error(error));
}

// ================ Intents =====================

function search() {
	let currentState = get(state);

	if (currentState.searchText.length === 0) {
		currentState.page = 0;
		currentState.filteredStore = currentState.store;
		currentState.displayedStore = currentState.filteredStore.slice(0, 12);

		state.set(currentState);
		return;
	}

	let searchTerm = currentState.searchText.toLowerCase();

	currentState.filteredStore = currentState.store
		.filter(
			(item) =>
				item.name.toLowerCase().includes(searchTerm) ||
				item.description.toLowerCase().includes(searchTerm)
		)
		.slice(0, 12);

	currentState.page = 0;

	currentState.displayedStore = currentState.filteredStore;

	state.set(currentState);
}

export async function onInstallExtension(repo: string) {
	try {
		let currentState = get(state);
		currentState.installingExtension = true;
		state.set(currentState);

		await invoke('run_clone_extension', { url: repo });
		currentState.installingExtension = false;

		currentState.installedExtensions = [];

		let extensions: ExtensionManifest[] = await invoke('run_get_extensions');
		extensions.forEach((extension) => {
			currentState.installedExtensions.push(extension.id);
		});

		currentState.store = await invoke('run_get_extensions_store');

		state.set(currentState);

		search();

		emit('refresh-extensions');
	} catch (error) {
		console.error(error);
	}
}

export function onSearchInput(search_text: CustomEvent<string>) {
	let currentState = get(state);

	currentState.searchText = search_text.detail;

	state.set(currentState);

	search();
}

export async function onGoToPreviousPage() {
	let currentState = get(state);

	currentState.page -= 1;

	currentState.displayedStore = currentState.filteredStore.slice(
		currentState.page * 12,
		(currentState.page + 1) * 12
	);

	state.set(currentState);
}

export async function onGoToNextPage() {
	let currentState = get(state);

	currentState.page += 1;

	currentState.displayedStore = currentState.filteredStore.slice(
		currentState.page * 12,
		(currentState.page + 1) * 12
	);

	state.set(currentState);
}
