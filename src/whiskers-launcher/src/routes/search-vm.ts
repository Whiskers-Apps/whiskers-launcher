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
	settings: {} as Settings
});

let results: WLResult[] = [];
export const displayedResults = writable<WLResult[]>([]);
let resultOffset = 0;
export const selectedIndex = writable(0);
export const searchText = writable('');
export const showConfirmationBox = writable(false);

export async function init() {
	let currentState = get(state);

	currentState.settings = await getSettings();
	currentState.css = getThemeCss(currentState.settings);
	currentState.loading = false;
	
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
	results = await invoke('get_results', { text: get(searchText) });

	selectedIndex.set(0);
	resultOffset = 0;

	cutResults();
}

export async function onSearchInput(
	event: Event & { currentTarget: EventTarget & HTMLInputElement }
) {
	searchText.set(event.currentTarget.value);
	onSearch();
}

export async function cutResults() {
	let currentSettings = get(state).settings;

	displayedResults.set(
		results.slice(resultOffset, resultOffset + currentSettings.results_count)
	);
}

export async function onBlur() {
	let currentSettings = get(state).settings;

	if (currentSettings.hide_on_blur) {
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
	let currentSelectedIndex = get(selectedIndex);
	let currentSettings = get(state).settings;

	showConfirmationBox.set(false);

	if (currentSelectedIndex > 0) {
		selectedIndex.set(currentSelectedIndex - 1);
		return;
	}

	if (resultOffset - 1 > 0) {
		resultOffset--;
		cutResults();
		return;
	}

	if (resultOffset === 0) {
		if (results.length < currentSettings.results_count) {
			selectedIndex.set(results.length - 1);
			return;
		}
	}

	resultOffset = results.length - currentSettings.results_count;
	selectedIndex.set(currentSettings.results_count - 1);

	cutResults();
}

export async function onGoToNextResult() {
	let currentSelectedIndex = get(selectedIndex);
	let currentDisplayedResults = get(displayedResults);
	let currentSettings = get(state).settings;

	showConfirmationBox.set(false);

	if (currentSelectedIndex < currentDisplayedResults.length - 1) {
		selectedIndex.set(currentSelectedIndex + 1);
		return;
	}

	if (resultOffset + currentSelectedIndex < results.length - 1) {
		resultOffset++;
		cutResults();
		return;
	}

	if (results.length < currentSettings.results_count) {
		if (currentSelectedIndex + 1 === results.length) {
			selectedIndex.set(0);
			return;
		}
	}

	resultOffset = 0;
	selectedIndex.set(0);

	cutResults();
}

export async function onSelectAltResult(index: number) {
	let currentDisplayedResults = get(displayedResults);

	if (index > currentDisplayedResults.length) {
		return;
	}

	selectedIndex.set(index);

	onRunAction();
}

export async function onRunAction() {
	let currentDisplayedResults = get(displayedResults);
	let currentSelectedIndex = get(selectedIndex);
	let currentShowConfirmationBox = get(showConfirmationBox);
	let result = currentDisplayedResults[currentSelectedIndex];

	if (!currentShowConfirmationBox) {
		if (
			(result.result_type === 'Text' && result.text?.action.ask_confirmation) ||
			(result.result_type === 'TitleAndDescription' &&
				result.title_and_description?.action.ask_confirmation)
		) {
			showConfirmationBox.set(true);
		} else {
			invoke('run_action', { result: currentDisplayedResults[currentSelectedIndex] });
		}
	} else {
		showConfirmationBox.set(false);
		invoke('run_action', { result: currentDisplayedResults[currentSelectedIndex] });
	}
}

export function onSetSelectedIndex(index: number) {
	selectedIndex.set(index);
}

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
