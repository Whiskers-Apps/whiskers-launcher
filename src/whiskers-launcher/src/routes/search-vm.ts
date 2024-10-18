import { invoke } from '@tauri-apps/api';
import { get, writable } from 'svelte/store';
import { appWindow } from '@tauri-apps/api/window';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import type { SearchResult, SearchResults } from '$lib/features/search/Search';
import { getSettings, type Settings } from '$lib/features/settings/Settings';
import { getCssFilter, getThemeCss } from '$lib/features/theming/Theming';

export const state = writable({
	loading: true,
	css: '',
	settings: {} as Settings,
	resultsType: '',
	results: [] as SearchResult[],
	resultsOffset: 0,
	resultsCount: 0,
	selectedIndex: 0,
	searchText: '',
	askConfirmation: false
});

export const colorFilter = writable('');
let maxDisplayedResultsCount = 9;

let searchResults = {} as SearchResults;

export async function init() {
	let currentState = get(state);

	currentState.settings = await getSettings();
	currentState.css = getThemeCss(currentState.settings);
	currentState.loading = false;

	colorFilter.set(getColorFilter(currentState.settings.theme.accent));

	state.set(currentState);

	onSearch();
}

// =============== Listeners =================
window.addEventListener('keydown', (event) => {
	switch (event.key) {
		case 'Escape': {
			appWindow.close();
			break;
		}

		case 'ArrowUp': {
			event.preventDefault();
			onGoUp();
			break;
		}

		case 'ArrowDown': {
			event.preventDefault();
			onGoDown();
			break;
		}

		case 'ArrowRight': {
			if (get(state).resultsType === 'Grid') {
				event.preventDefault();
				onGoRight();
			}
			break;
		}

		case 'ArrowLeft': {
			if (get(state).resultsType === 'Grid') {
				event.preventDefault();
				onGoLeft();
			}

			break;
		}

		case 'Enter': {
			onRunAction();
			break;
		}
	}

	if (event.ctrlKey && event.key === 's') {
		onOpenSettings();
	}

	if (event.altKey && ['1', '2', '3', '4', '5', '6', '7', '8', '9'].includes(event.key)) {
		if (get(state).settings.launch_key === 'Alt') {
			onSelectResult(+event.key - 1);
		}
	}

	if (event.ctrlKey && ['1', '2', '3', '4', '5', '6', '7', '8', '9'].includes(event.key)) {
		if (get(state).settings.launch_key === 'Ctrl') {
			onSelectResult(+event.key - 1);
		}
	}
});

// =============== Actions ===================

export async function onSearch() {
	let newState = get(state);

	searchResults = await invoke('run_get_search_results', { search_text: newState.searchText });

	newState.selectedIndex = 0;
	newState.resultsCount = searchResults.results.length;
	newState.resultsOffset = 0;
	newState.resultsType = searchResults.view_type;

	maxDisplayedResultsCount = searchResults.view_type === 'Grid' ? 12 : 9;

	state.set(newState);

	cutResults();
}

export async function onSearchInput(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);

	currentState.searchText = event.currentTarget.value;

	state.set(currentState);

	onSearch();
}

export async function cutResults() {
	let currentState = get(state);
	let offset = currentState.resultsOffset;

	currentState.results = searchResults.results.slice(offset, offset + maxDisplayedResultsCount);

	state.set(currentState);
}

export async function onBlur() {
	let currentSettings = get(state).settings;

	if (currentSettings.hide_on_blur) {
		appWindow.close();
	}
}

export async function onOpenSettings(event: Event | undefined = undefined) {
	event?.stopPropagation();

	invoke('run_open_settings_window');

	setTimeout(() => {
		appWindow.close();
	}, 200);
}

function getIndexColumn(index: number): number {
	if ([0, 4, 8].includes(index)) return 0;
	if ([1, 5, 9].includes(index)) return 1;
	if ([2, 6, 10].includes(index)) return 2;
	return 3;
}

function getMaxIndexColumn(results: SearchResult[]): number {
	let resultsLength = results.length;

	if ([1, 5, 9].includes(resultsLength)) return 0;
	if ([2, 6, 10].includes(resultsLength)) return 1;
	if ([3, 7, 11].includes(resultsLength)) return 2;
	return 3;
}

export async function onGoUp() {
	let currentState = get(state);
	currentState.askConfirmation = false;

	switch (currentState.resultsType) {
		case 'Grid': {
			if (currentState.selectedIndex === 0 && searchResults.results.length <= 4) {
				return;
			}

			if (currentState.selectedIndex - 4 >= 0) {
				currentState.selectedIndex = currentState.selectedIndex - 4;
				state.set(currentState);
				return;
			}
			if (currentState.resultsOffset - 4 > 0) {
				currentState.resultsOffset = currentState.resultsOffset - 4;
				state.set(currentState);
				cutResults();
				return;
			}
			if (currentState.resultsOffset === 0) {
				if (searchResults.results.length < 12) {
					currentState.selectedIndex =
						searchResults.results.length - getMaxIndexColumn(currentState.results);

					state.set(currentState);
					return;
				}
			}

			currentState.resultsOffset = searchResults.results.length - 12;
			currentState.selectedIndex = getMaxIndexColumn(currentState.results) + 8;
			state.set(currentState);
			break;
		}
		default: {
			if (currentState.selectedIndex > 0) {
				currentState.selectedIndex = currentState.selectedIndex - 1;
				state.set(currentState);
				return;
			}

			if (currentState.resultsOffset - 1 > 0) {
				currentState.resultsOffset = currentState.resultsOffset - 1;
				state.set(currentState);
				cutResults();
				return;
			}

			if (currentState.resultsOffset === 0) {
				if (searchResults.results.length < 9) {
					currentState.selectedIndex = searchResults.results.length - 1;
					state.set(currentState);
					return;
				}
			}

			currentState.resultsOffset = searchResults.results.length - 9;
			currentState.selectedIndex = 8;
			state.set(currentState);
			break;
		}
	}

	cutResults();
}

export async function onGoDown() {
	let currentState = get(state);
	currentState.askConfirmation = false;

	switch (currentState.resultsType) {
		case 'Grid': {
			if (currentState.selectedIndex <= 3 && searchResults.results.length <= 4) {
				return;
			}

			if (currentState.selectedIndex + 4 < currentState.results.length) {
				currentState.selectedIndex = currentState.selectedIndex + 4;
				state.set(currentState);
				return;
			}

			if (
				currentState.selectedIndex + 4 < searchResults.results.length - 1 &&
				searchResults.results.length - (currentState.resultsOffset + 4) >= 12
			) {
				currentState.selectedIndex = getIndexColumn(currentState.selectedIndex);
				currentState.resultsOffset = currentState.resultsOffset + 4;
				state.set(currentState);
				cutResults();
				return;
			}

			if (searchResults.results.length < 12) {
				if (currentState.selectedIndex + 4 === searchResults.results.length) {
					currentState.selectedIndex = 0;
					state.set(currentState);
					return;
				}
			}

			currentState.resultsOffset = 0;
			currentState.selectedIndex = 0;
			state.set(currentState);
			break;
		}
		default: {
			if (currentState.selectedIndex < currentState.results.length - 1) {
				currentState.selectedIndex = currentState.selectedIndex + 1;
				state.set(currentState);
				return;
			}

			if (
				currentState.resultsOffset + currentState.selectedIndex <
				searchResults.results.length - 1
			) {
				currentState.resultsOffset = currentState.resultsOffset + 1;
				state.set(currentState);
				cutResults();
				return;
			}

			if (searchResults.results.length < 9) {
				if (currentState.selectedIndex + 1 === searchResults.results.length) {
					currentState.selectedIndex = 0;
					state.set(currentState);
					return;
				}
			}

			currentState.resultsOffset = 0;
			currentState.selectedIndex = 0;
			state.set(currentState);
			break;
		}
	}

	cutResults();
}

function onGoLeft() {
	let currentState = get(state);
	let column = getIndexColumn(currentState.selectedIndex);

	if (column - 1 >= 0) {
		currentState.selectedIndex = currentState.selectedIndex - 1;
		state.set(currentState);
	}

	cutResults();
}

function onGoRight() {
	let currentState = get(state);
	let column = getIndexColumn(currentState.selectedIndex);

	if (column + 1 < 4 && currentState.selectedIndex + 1 < currentState.results.length) {
		currentState.selectedIndex = currentState.selectedIndex + 1;
		state.set(currentState);
	}

	cutResults();
}

export async function onSelectResult(index: number) {
	let currentState = get(state);

	onRunAction();
}

export async function onRunAction() {
	let currentState = get(state);
	let result = currentState.results[currentState.selectedIndex];

	if (!currentState.askConfirmation) {
		if (result.action.dangerous) {
			currentState.askConfirmation = true;
			state.set(currentState);
		} else {
			invoke('run_action', { action: result.action });
		}
	} else {
		currentState.askConfirmation = false;
		state.set(currentState);

		invoke('run_action', { action: result.action });
	}
}

export function onSetSelectedIndex(index: number) {
	let currentState = get(state);
	currentState.selectedIndex = index;
	state.set(currentState);
}

/* ========================== Utils =============================== */

export function getColorFilter(tint: string | null): string {
	let currentSettings = get(state).settings;

	if (tint === null) {
		return 'none';
	}

	if (tint === 'accent') {
		return getCssFilter(currentSettings.theme.accent);
	}

	return getCssFilter(tint);
}

export function getIconPath(iconPath: string | null | undefined): string | null {
	if (iconPath === null || iconPath === undefined) {
		return null;
	}

	try {
		let src = convertFileSrc(iconPath ? iconPath : '');

		return src;
	} catch {
		return null;
	}
}

export function getMainClasses(): string {
	let settings = get(state).settings;

	if (settings.wallpaper !== null) {
		return 'wallpaper h-screen w-full flex items-center justify-center';
	} else {
		return '';
	}
}

export function isSelected(index: number): boolean {
	return index == get(state).selectedIndex;
}
