import { getSettings, writeSettings, type Settings } from '$lib/settings/settings';
import { WebviewWindow } from '@tauri-apps/api/window';
import { get, writable } from 'svelte/store';
import { WindowSizes } from '../../../utils';
import { listen } from '@tauri-apps/api/event';

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

export function onSetSearchKeyword(keyword: CustomEvent<string>) {
	let currentState = get(state);
	currentState.settings.search_keyword = keyword.detail;
	state.set(currentState);

	writeSettings(currentState.settings);
}

async function refreshSettings() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	state.set(currentState);
}

export async function onAddSearchEngine() {
	new WebviewWindow('add-search-engine', {
		url: 'settings/search-engines/add-search-engine',
		title: 'Add Search Engine',
		resizable: false,
		width: WindowSizes.SearchEngine.width,
		height: WindowSizes.SearchEngine.height,
		center: true
	});

	const unlisten = await listen('refresh-search-engines', async (_) => {
		refreshSettings();
		unlisten();
	});
}

function closeMenus() {
	get(state).settings.search_engines.forEach((engine) => {
		let element = document.getElementById(`menu-${engine.id}`);
		element?.classList.add('hidden');
		element?.classList.remove('flex');
	});
}

export function onOpenSearchEngineMenu(id: number) {
	closeMenus();

	let buttonRect = document.getElementById(`menu-button-${id}`)!!.getBoundingClientRect();
	let menu = document.getElementById(`menu-${id}`)!!;
	menu.style.left = `${buttonRect.left - 100}px`;
	menu.style.top = `${buttonRect.top}px`;
	menu.classList.add('flex');
	menu.classList.remove('hidden');
}

export function onSetDefaultSearchEngine(id: number) {
    closeMenus();

	let currentState = get(state);
	currentState.settings.default_search_engine = id;
	state.set(currentState);

	writeSettings(currentState.settings);
}

export async function onEditSearchEngine(id: number) {
    closeMenus();

	new WebviewWindow('edit-search-engine', {
		url: `settings/search-engines/edit-search-engine?id=${id}`,
		title: 'Edit Search Engine',
		resizable: false,
		width: WindowSizes.SearchEngine.width,
		height: WindowSizes.SearchEngine.height,
		center: true
	});

	const unlisten = await listen('refresh-search-engines', async (_) => {
		refreshSettings();
		unlisten();
	});
}

export async function onDeleteSearchEngine(id: number) {
    closeMenus();

	new WebviewWindow('delete-search-engine', {
		url: `settings/search-engines/delete-search-engine/?id=${id}`,
		title: 'Delete Search Engine',
		resizable: false,
		width: WindowSizes.ConfirmDialog.width,
		height: WindowSizes.ConfirmDialog.height,
		center: true
	});

	const unlisten = await listen('refresh-search-engines', async (_) => {
		refreshSettings()
		unlisten();
	});
}
