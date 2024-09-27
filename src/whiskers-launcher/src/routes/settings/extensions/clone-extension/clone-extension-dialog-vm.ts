import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import { get, writable } from 'svelte/store';

const expression =
	/[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)/gi;

const urlRegex = new RegExp(expression);

export const state = writable({
	url: '',
	disableCloneButton: true,
	cloning: false
});

export function onSetUrl(url: CustomEvent<string>) {
	let currentState = get(state);
	currentState.url = url.detail;
	currentState.disableCloneButton = !urlRegex.test(currentState.url);
	state.set(currentState);
}

export async function onClone() {
	let currentState = get(state);
	currentState.disableCloneButton = true;
	currentState.cloning = true;
	state.set(currentState);

	await invoke('clone_extension', { url: currentState.url });

	await emit('refresh-extensions');

	appWindow.close();
}
