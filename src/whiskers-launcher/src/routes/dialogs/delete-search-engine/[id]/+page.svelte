<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/dialog.svelte';
	import { invoke } from '@tauri-apps/api';
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	let id = +$page.params.id;

	async function onClick() {
		await invoke('delete_search_engine', { id: id });
		await emit('refresh-search-engines');
		appWindow.close();
	}
</script>

<Dialog
	title="Delete Search Engine"
	description="Are you sure you want to delete the search engine?"
	actionText="Delete"
	on:click={onClick}
/>
