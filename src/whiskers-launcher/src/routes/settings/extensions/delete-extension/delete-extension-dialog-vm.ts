import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

export async function onRemoveApp(id: string) {
	await invoke('run_remove_extension', { id: id });
	await emit('refresh-extensions');

	appWindow.close();
}
