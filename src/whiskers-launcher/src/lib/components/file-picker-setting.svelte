<script lang="ts">
	import DocumentIcon from '$lib/icons/document.svg?component';
	import TrashIcon from '$lib/icons/trash.svg?component';
	import { createEventDispatcher } from 'svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { homeDir } from '@tauri-apps/api/path';

	export let title: string = '';
	export let description: string = '';
	export let selectedPath: string | null = null;
	export let fileTypes: string[] | null = null;

	let dispatch = createEventDispatcher();

	function onFileSelected(path: string) {
		dispatch('file-changed', path);
	}

	function onClearFile() {
		dispatch('file-changed');
	}

	async function onSelectFile() {
		const path = await open({
			defaultPath: selectedPath ?? (await homeDir()),
			filters:
				fileTypes !== null
					? [
							{
								name: 'Supported Types',
								extensions: fileTypes
							}
						]
					: undefined
		});

		if (path) {
			onFileSelected(path as string);
		}
	}
</script>

<div class="space-y-2">
	<div>
		<p class=" text-xl font-medium">{title}</p>
		<p class=" text-sub-text">{description}</p>
	</div>
	<div class="flex items-center card">
		<button class="flex items-center w-full" on:click={onSelectFile}>
			<DocumentIcon class="h-6 w-6" />
			{#if selectedPath === null}
				<div class="ml-2">
					<p>Select a file.</p>
				</div>
			{:else}
				<div class="ml-2">
					<p>{selectedPath}</p>
				</div>
			{/if}
		</button>
		{#if selectedPath !== null}
			<button
				class=" p-2 hover-bg-tertiary text-danger rounded-full hover-text-on-danger hover-bg-danger"
				on:click={onClearFile}
			>
				<TrashIcon class="h-6 w-6" />
			</button>
		{/if}
	</div>
</div>
