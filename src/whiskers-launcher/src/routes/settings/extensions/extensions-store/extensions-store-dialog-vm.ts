import type { Extension, ExtensionStoreItem } from '$lib/settings/settings';
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

	let extensions: Extension[] = await invoke('get_extensions');
	extensions.forEach((extension) => {
		currentState.installedExtensions.push(extension.id);
	});

	let os: string = await invoke('get_os');

	currentState.store = await invoke('get_extensions_store');
	currentState.store = currentState.store.filter((extension) => extension?.os?.includes(os));
	currentState.filteredStore = currentState.store;
	currentState.displayedStore = currentState.store.slice(0, 12);
	currentState.loading = false;

	state.update(() => currentState);

	axios
		.get(
			'https://raw.githubusercontent.com/Whiskers-Apps/whiskers-launcher-extensions/main/extensions.json'
		)
		.then((response) => {
			currentState.store = response.data;
			currentState.filteredStore = currentState.store;
			currentState.displayedStore = currentState.store.slice(0, 12);

			invoke('write_extensions_store', { store: currentState.store });

			state.update(() => currentState);
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

		state.update(() => currentState);
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

	state.update(() => currentState);
}

export async function onInstallExtension(repo: string) {
	try {
		let currentState = get(state);
		currentState.installingExtension = true;
		state.update(() => currentState);

		await invoke('clone_extension', { url: repo });
		currentState.installingExtension = false;

		currentState.installedExtensions = [];

		let extensions: Extension[] = await invoke('get_extensions');
		extensions.forEach((extension) => {
			currentState.installedExtensions.push(extension.id);
		});

		currentState.store = await invoke('get_extensions_store');

		state.update(() => currentState);

		search();

		emit('refresh-extensions');
	} catch (error) {
		console.error(error);
	}
}

export function onSearchInput(search_text: CustomEvent<string>) {
	let currentState = get(state);

	currentState.searchText = search_text.detail;

	state.update(() => currentState);

	search();
}

export async function onGoToPreviousPage() {
	let currentState = get(state);

	currentState.page -= 1;

	currentState.displayedStore = currentState.filteredStore.slice(
		currentState.page * 12,
		(currentState.page + 1) * 12
	);

	state.update(() => currentState);
}

export async function onGoToNextPage() {
	let currentState = get(state);

	currentState.page += 1;

	currentState.displayedStore = currentState.filteredStore.slice(
		currentState.page * 12,
		(currentState.page + 1) * 12
	);

	state.update(() => currentState);
}
