import { getSettings, type SearchEngine } from '$lib/settings/settings';
import { emit } from '@tauri-apps/api/event';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { get, writable } from 'svelte/store';
import { open } from '@tauri-apps/api/dialog';

export const state = writable<{
	loading: boolean;
	iconPath: string | null;
	convertedIconPath: string | null;
	tintIcon: boolean;
	keyword: string;
	name: string;
	searchQuery: string;
	id: number;
}>({
	loading: true,
	iconPath: null,
	convertedIconPath: null,
	tintIcon: false,
	keyword: '',
	name: '',
	searchQuery: '',
	id: 0
});

export async function init(id: number) {
	let currentState = get(state);
	currentState.id = id;

	let settings = await getSettings();
	let engine = settings.search_engines.find((engine) => engine.id === id)!!;

	currentState.iconPath = engine.icon_path;
	currentState.convertedIconPath = currentState.iconPath
		? `${convertFileSrc(currentState.iconPath!!)}?${Math.floor(Math.random() * 696969)}`
		: null;

	currentState.tintIcon = engine.tint_icon;
	currentState.keyword = engine.keyword;
	currentState.name = engine.name;
	currentState.searchQuery = engine.search_query;

	currentState.loading = false;

	state.update(() => currentState);
}

// ================ Intents =============================

export async function getIcon() {
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
		let currentState = get(state);
		
		currentState.iconPath = path.toString();
		currentState.convertedIconPath = `${convertFileSrc(path.toString())}?${Math.floor(Math.random() * 696969)}`;
		
		state.update(() => currentState);
	}
}

export function onClearIcon() {
	let currentState = get(state);
	currentState.iconPath = null;
	currentState.convertedIconPath = null;
	state.update(() => currentState);
}

export function onSetTintIcon(tint: CustomEvent<boolean>){
	let currentState = get(state);
	currentState.tintIcon = tint.detail;
	state.update(() => currentState);
}

export function onSetKeyword(event: CustomEvent<string>) {
	let currentState = get(state);
	currentState.keyword = event.detail;
	state.update(() => currentState);
}

export function onSetName(event: CustomEvent<string>) {
	let currentState = get(state);
	currentState.name = event.detail;
	state.update(() => currentState);
}

export function onSetSearchQuery(event: CustomEvent<string>) {
	let currentState = get(state);
	currentState.searchQuery = event.detail;
	state.update(() => currentState);
}

export async function onSave() {
	let currentState = get(state);

	let engine: SearchEngine = {
		id: currentState.id,
		icon_path: currentState.iconPath,
		tint_icon: currentState.tintIcon,
		keyword: currentState.keyword,
		name: currentState.name,
		search_query: currentState.searchQuery
	};

	await invoke('update_search_engine', { engine: engine });
	await emit('refresh-settings');
	await emit('refresh-search-engines');

	appWindow.close();
}
