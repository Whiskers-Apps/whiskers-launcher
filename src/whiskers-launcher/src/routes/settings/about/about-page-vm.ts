import type { ExtensionManifest } from '$lib/features/extensions/Extensions';
import { invoke } from '@tauri-apps/api';
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/api/shell';
import { get, writable } from 'svelte/store';

export const GITHUB_REPO_URL = 'https://github.com/Whiskers-Apps/whiskers-launcher/';

export const state = writable({
	loading: true,
	appVersion: '',
	extensionsCount: 0
});

export async function init() {
	let extensions: ExtensionManifest[] = await invoke('run_get_extensions');

	let currentState = get(state);
	currentState.appVersion = await getVersion();
	currentState.extensionsCount = extensions.length;
	currentState.loading = false;

	state.set(currentState);
}

// =============== Intents ===================== //

export function onOpenGitHubRepo() {
	open(GITHUB_REPO_URL);
}

export function onOpenBuyMeACoffeePage(){
	open("https://buymeacoffee.com/lighttigerxiv")
}