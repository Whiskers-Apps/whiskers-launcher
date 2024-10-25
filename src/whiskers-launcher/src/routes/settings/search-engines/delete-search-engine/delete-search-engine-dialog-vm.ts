import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

export async function onDelete(id: number) {
	await invoke('run_delete_search_engine', { id: id });
	await emit('refresh-search-engines');
	appWindow.close();
}
