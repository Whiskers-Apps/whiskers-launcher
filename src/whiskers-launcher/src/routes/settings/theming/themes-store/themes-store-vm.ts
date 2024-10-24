import type { SelectValue } from '$lib/components/classes';
import { invoke } from '@tauri-apps/api';
import axios from 'axios';
import { get, writable } from 'svelte/store';
import { setDialogFrameCSS } from '../../../dialog-frame-vm';
import { emit } from '@tauri-apps/api/event';
import { getSettings, writeSettings, type ThemeStoreItem, type ThemeStoreVariant } from '$lib/features/settings/Settings';
import { getThemeCss } from '$lib/features/theming/Theming';

export const state = writable({
	loading: true,
	searchText: '',
	store: [] as ThemeStoreItem[],
	filteredStore: [] as ThemeStoreItem[],
	displayedStore: [] as ThemeStoreItem[],
	page: 0,
	applyingTheme: false,
	listingSelectValues: [] as ListingSelectValue[]
});

interface ListingSelectValue {
	theme_id: string;
	value_id: string;
}

export async function init() {
	let currentState = get(state);
	currentState.store = await invoke('run_get_themes_store');
	currentState.filteredStore = currentState.store;
	currentState.displayedStore = currentState.store.slice(0, 12);

	state.set(currentState);

	indexSelectValues();

	currentState.loading = currentState.store.length === 0;
	state.set(currentState);

	await axios
		.get(
			'https://raw.githubusercontent.com/Whiskers-Apps/whiskers-launcher-themes/master/themes.json'
		)
		.then((response) => {
			currentState.store = response.data;
			currentState.filteredStore = currentState.store;
			currentState.displayedStore = currentState.store.slice(0, 12);
			currentState.loading = false;

			state.set(currentState);

			indexSelectValues();

			invoke('run_write_themes_store', { store: currentState.store });
		})
		.catch((error) => console.error(error));
}

// ================ Intents =====================

async function indexSelectValues() {
	let currentState = get(state);
	let selectValues: ListingSelectValue[] = [];
	currentState.store.forEach((listing) => {
		if (listing.variants) {
			selectValues.push({ theme_id: listing.id, value_id: listing.variants!![0].file });
		}
	});

	currentState.listingSelectValues = selectValues;
	state.set(currentState);
}

async function search() {
	let currentState = get(state);
	if (currentState.searchText.length === 0) {
		currentState.page = 0;
		currentState.filteredStore = currentState.store;
		currentState.displayedStore = [...currentState.filteredStore.slice(0, 12)];

		state.set(currentState);
		return;
	}

	let searchTerm = currentState.searchText.toLowerCase();

	currentState.filteredStore = currentState.store
		.filter((item) => item.name.toLowerCase().includes(searchTerm))
		.slice(0, 12);

	currentState.page = 0;

	currentState.displayedStore = currentState.filteredStore;

	state.set(currentState);
}

export function getSelectValues(variants: ThemeStoreVariant[]): SelectValue[] {
	return variants.map((variant) => ({
		id: variant.file,
		value: variant.name
	}));
}

export function getSelectValueId(id: string): string {
	let currentState = get(state);

	for (let value of currentState.listingSelectValues) {
		if (value.theme_id === id) {
			return value.value_id;
		}
	}

	return '';
}

export function onSelectAccent(theme_id: string, value_id: string) {
	let currentState = get(state);

	let newListing = currentState.listingSelectValues.map((value) =>
		value.theme_id === theme_id ? { theme_id: theme_id, value_id: value_id } : value
	);

	currentState.listingSelectValues = newListing;

	state.set(currentState);
}

export async function applyTheme(theme: ThemeStoreItem) {
	try {
		let currentState = get(state);
		currentState.applyingTheme = true;
		state.set(currentState);

		let repo = theme.file ? theme.file : getSelectValueId(theme.id);

		axios
			.get(repo)
			.then(async (response) => {
				let json = response.data;
				let settings = await getSettings();
				settings.theme = {
					background: json.background,
					secondary: json.secondary,
					tertiary: json.tertiary,
					accent: json.accent,
					warning: json.warning,
					danger: json.danger,
					on_accent: json.on_accent,
					on_danger: json.on_danger,
					text: json.text,
					sub_text: json.sub_text
				};

				writeSettings(settings);
				setDialogFrameCSS(getThemeCss(settings));

				emit('refresh-theme');
			})
			.catch((error) => console.error(error))
			.finally(() => {
				currentState.applyingTheme = false;
				state.set(currentState);
			});
	} catch (error) {
		console.error(error);
	}
}

export async function goToPreviousPage() {
	let currentState = get(state);

	currentState.page -= 1;

	currentState.displayedStore = currentState.filteredStore.slice(
		currentState.page * 12,
		(currentState.page + 1) * 12
	);

	state.set(currentState);
}

export async function goToNextPage() {
	let currentState = get(state);

	currentState.page += 1;

	currentState.displayedStore = currentState.filteredStore.slice(
		currentState.page * 12,
		(currentState.page + 1) * 12
	);

	state.set(currentState);
}

export function onSearchInput(searchText: CustomEvent<string>) {
	let currentState = get(state);
	currentState.searchText = searchText.detail;
	state.set(currentState);

	search();
}
