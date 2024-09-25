import {
	getSettings,
	writeSettings,
	type Extension,
	type ExtensionSetting,
	type Settings
} from '$lib/settings/settings';
import { invoke } from '@tauri-apps/api';
import { WebviewWindow } from '@tauri-apps/api/window';
import { get, writable } from 'svelte/store';
import { WindowSizes } from '../../../utils';
import { listen } from '@tauri-apps/api/event';

export const state = writable({
	loading: false,
	settings: {} as Settings,
	os: '',
	extensions: [] as Extension[]
});

export async function init() {
	let currentState = get(state);
	currentState.settings = await getSettings();
	currentState.os = await invoke('get_os');
	currentState.extensions = await invoke('get_extensions');
	currentState.loading = false;

	state.update(() => currentState);
}

// =================== Intents ====================

export function onOpenStore() {
	new WebviewWindow('extensions-store', {
		url: 'settings/extensions/extensions-store',
		title: 'Extensions Store',
		height: WindowSizes.Store.height,
		width: WindowSizes.Store.width,
		resizable: false,
		maximizable: false
	});

	listen('refresh-extensions', () => {
		reloadExtensions();
	});
}

export async function onOpenCloneExtensionDialog() {
	new WebviewWindow('clone-extension', {
		url: 'settings/extensions/clone-extension',
		title: 'Clone Extension',
		height: WindowSizes.CloneExtension.height,
		width: WindowSizes.CloneExtension.width,
		resizable: false,
		maximizable: false
	});

	const unlisten = await listen('refresh-extensions', async () => {
		await reloadExtensions();
		unlisten();
	});
}

async function reloadExtensions() {
	await invoke('index_extensions');
	let currentState = get(state);
	currentState.extensions = await invoke('get_extensions');
	state.update(() => currentState);
}

export function onReloadExtensions() {
	reloadExtensions();
}

export function onOpenExtensionDir(id: string) {
	invoke('open_extension_dir', { id: id });
}

export async function onDeleteExtension(id: string) {
	new WebviewWindow('delete-extension', {
		url: `settings/extensions/delete-extension?id=${encodeURIComponent(id)}`,
		title: 'Delete Extension',
		height: WindowSizes.ConfirmDialog.height,
		width: WindowSizes.ConfirmDialog.width,
		resizable: false,
		maximizable: false
	});

	const unlisten = await listen('refresh-extensions', async () => {
		reloadExtensions();
		unlisten();
	});
}

export function getExtensionSettingValue(id: string, setting: string): string {
	let extensionsSettings = get(state).settings.extensions;

	for (const index in extensionsSettings) {
		if (
			extensionsSettings[index].extension_id === id &&
			extensionsSettings[index].setting_id === setting
		) {
			return extensionsSettings[index].setting_value;
		}
	}

	return '';
}

export async function onUpdateSetting(extensionId: string, settingId: string, value: string) {
	let newExtensionsSettings: ExtensionSetting[] = [];
	let currentState = get(state);

	currentState.settings.extensions.forEach((extensionSetting) => {
		if (
			extensionSetting.extension_id === extensionId &&
			extensionSetting.setting_id === settingId
		) {
			let setting = extensionSetting;
			setting.setting_value = value;
			newExtensionsSettings.push(setting);
		} else {
			newExtensionsSettings.push(extensionSetting);
		}
	});

	currentState.extensions = await invoke('get_extensions');
	state.update(() => currentState);

	writeSettings(currentState.settings);
}

export function canShowSetting(extensionId: string, setting: ExtensionSetting): boolean {
	if (setting.os !== '*' && setting.os.toLowerCase() != get(state).os) {
		return false;
	}

	if (setting.show_conditions !== null) {
		for (const index in setting.show_conditions) {
			const condition = setting.show_conditions[index];

			if (condition.setting_value !== getExtensionSettingValue(extensionId, condition.setting_id)) {
				return false;
			}
		}
	}

	return true;
}
