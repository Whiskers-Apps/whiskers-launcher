import {
	getCssFilter,
	getSettings,
	getThemeCss,
	type Settings,
	type WLResult
} from '$lib/settings/settings';
import { invoke } from '@tauri-apps/api';
import { get, writable } from 'svelte/store';
import { appWindow } from '@tauri-apps/api/window';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export const state = writable({
	loading: true,
	css: '',
	settings: {} as Settings,
	results: [] as WLResult[],
	displayedResults: [] as WLResult[],
	selectedIndex: 0,
	resultOffset: 0,
	searchText: '',
	showConfirmationBox: false
});

export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.css = getThemeCss(currentState.settings);
	currentState.loading = false;
	state.update(() => currentState);

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
			onGoToPreviousResult();
			break;
		}
		case 'ArrowDown': {
			event.preventDefault();
			onGoToNextResult();
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
		onSelectAltResult(+event.key - 1);
	}
});

// =============== Intents ===================

export async function onSearch() {
	let currentState = get(state);
	currentState.results = await invoke('get_results', { text: currentState.searchText });
	currentState.selectedIndex = 0;
	currentState.resultOffset = 0;

	state.update(() => currentState);

	cutResults();
}

export async function onSearchInput(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	let currentState = get(state);
	currentState.searchText = event.currentTarget.value;
	state.update(() => currentState);
	onSearch();
}

export async function cutResults() {
	let currentState = get(state);
	currentState.displayedResults = [
		...currentState.results.slice(
			currentState.resultOffset,
			currentState.resultOffset + currentState.settings.results_count
		)
	];

	state.update(() => currentState);
}

export async function onBlur() {
	if (get(state).settings.hide_on_blur) {
		appWindow.close();
	}
}

export async function onOpenSettings(event: Event | undefined = undefined) {
	event?.stopPropagation();

	invoke('open_settings_window');

	setTimeout(() => {
		appWindow.close();
	}, 200);
}

export async function onGoToPreviousResult() {
	let currentState = get(state);
	currentState.showConfirmationBox = false;

	if (currentState.selectedIndex > 0) {
		currentState.selectedIndex--;
		state.update(() => currentState);
		return;
	}

	if (currentState.resultOffset - 1 > 0) {
		currentState.resultOffset--;
		state.update(() => currentState);
		cutResults();
		return;
	}

	if (currentState.resultOffset === 0) {
		if (currentState.results.length < currentState.settings.results_count) {
			currentState.selectedIndex = currentState.results.length - 1;
			state.update(() => currentState);
			return;
		}
	}

	currentState.resultOffset = currentState.results.length - currentState.settings.results_count;
	currentState.selectedIndex = currentState.settings.results_count - 1;

	state.update(() => currentState);

	cutResults();
}

export async function onGoToNextResult() {
	let currentState = get(state);
	currentState.showConfirmationBox = false;

	if (currentState.selectedIndex < currentState.displayedResults.length - 1) {
		currentState.selectedIndex++;
		state.update(() => currentState);
		return;
	}

	if (currentState.resultOffset + currentState.selectedIndex < currentState.results.length - 1) {
		currentState.resultOffset++;
		state.update(() => currentState);
		cutResults();
		return;
	}

	if (currentState.results.length < currentState.settings.results_count) {
		if (currentState.selectedIndex + 1 === currentState.results.length) {
			currentState.selectedIndex = 0;
			state.update(() => currentState);
			return;
		}
	}

	currentState.resultOffset = 0;
	currentState.selectedIndex = 0;

	state.update(() => currentState);

	cutResults();
}

export async function onSelectAltResult(index: number) {
	let currentState = get(state);

	if (index > currentState.displayedResults.length) {
		return;
	}

	currentState.selectedIndex = index;

	state.update(() => currentState);

	onRunAction();
}

export async function onRunAction() {
	let currentState = get(state);
	let result = currentState.displayedResults[currentState.selectedIndex];

	if (!currentState.showConfirmationBox) {
		if (
			(result.result_type === 'Text' && result.text?.action.ask_confirmation) ||
			(result.result_type === 'TitleAndDescription' &&
				result.title_and_description?.action.ask_confirmation)
		) {
			currentState.showConfirmationBox = true;
		} else {
			invoke('run_action', { result: currentState.displayedResults[currentState.selectedIndex] });
		}
	} else {
		currentState.showConfirmationBox = false;
		invoke('run_action', { result: currentState.displayedResults[currentState.selectedIndex] });
	}

	state.update(() => currentState);
}

export function onSetSelectedIndex(index: number) {
	let currentState = get(state);
	currentState.selectedIndex = index;
	state.update(() => currentState);
}

// =============== UI =====================

export function getColorFilter(tint: string | null): string {
	if (tint === null) {
		return 'none';
	}

	if (tint === 'accent') {
		return getCssFilter(get(state).settings.theme.accent);
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
