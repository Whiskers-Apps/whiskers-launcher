import { get, writable } from 'svelte/store';
import { open } from '@tauri-apps/api/dialog';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import type { SearchEngine } from '$lib/settings/settings';
import { emit } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';

export const state = writable<{
	iconPath: string | null;
	convertedIconPath: string | null;
	tintIcon: boolean;
	keyword: string;
	name: string;
	searchQuery: string;
}>({
	iconPath: null,
	convertedIconPath: null,
	tintIcon: false,
	keyword: '',
	name: '',
	searchQuery: ''
});

export async function onSelectIcon() {
	let currentState = get(state);

	const path = await open({
		multiple: false,
		filters: [
			{
				name: 'Images',
				extensions: ['png', 'jpg', 'jpeg', 'svg']
			}
		]
	});

	if (path !== null) {
		currentState.iconPath = path.toString();
		currentState.convertedIconPath = `${convertFileSrc(path.toString())}?${Math.floor(Math.random() * 696969)}`;
	}

	state.set(currentState);
}

export function onClearIcon() {
	let currentState = get(state);
	currentState.iconPath = null;
	currentState.convertedIconPath = null;
	state.set(currentState);
}

export function onSetTintIcon(event: CustomEvent<boolean>) {
	let currentState = get(state);
	currentState.tintIcon = event.detail;
	state.set(currentState);
}

export function onSetKeyword(event: CustomEvent<string>) {
	let currentState = get(state);
	currentState.keyword = event.detail;
	state.set(currentState);
}

export function onSetName(event: CustomEvent<string>) {
	let currentState = get(state);
	currentState.name = event.detail;
	state.set(currentState);
}

export function onSetSearchQuery(event: CustomEvent<string>) {
	let currentState = get(state);
	currentState.searchQuery = event.detail;
	state.set(currentState);
}

export async function addSearchEngine() {
	let currentState = get(state);

	let engine: SearchEngine = {
		id: await invoke('get_new_search_engine_id'),
		icon_path: currentState.iconPath,
		tint_icon: currentState.tintIcon,
		keyword: currentState.keyword,
		name: currentState.name,
		search_query: currentState.searchQuery
	};

	await invoke('add_search_engine', { engine: engine });

	await emit('refresh-search-engines');

	appWindow.close();
}
