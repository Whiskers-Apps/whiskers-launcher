import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';

export async function onRemoveAppFromBlacklist(id: string) {
	await invoke('run_remove_from_blacklist', { id: id });
	await emit('refresh-blacklist');

	appWindow.close();
}
