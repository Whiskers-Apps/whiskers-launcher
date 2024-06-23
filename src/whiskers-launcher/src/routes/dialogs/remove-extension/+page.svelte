<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/dialog.svelte';
	import { invoke } from '@tauri-apps/api';
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	
	let id = decodeURIComponent($page.url.searchParams.get('id')!!);

	async function removeApp() {
		await invoke('remove_extension', { id: id });
		await emit('refresh-extensions');

		appWindow.close();
	}
</script>

<Dialog
	title="Remove Extension"
	description="Are you sure you want to remove this extension?"
	actionText="Remove"
	on:click={removeApp}
/>