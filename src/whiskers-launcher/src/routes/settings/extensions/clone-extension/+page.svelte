<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import DialogFrame from '../../../dialog-frame.svelte';
	import DownloadIcon from '$lib/icons/download.svg?component';
	import { onClone, onSetUrl, state } from './clone-extension-dialog-vm';

	$: uiState = $state;
</script>

<DialogFrame>
	<div class=" gap-4 h-screen w-screen p-4 flex flex-col justify-center overflow-hidden">
		{#if !uiState.cloning}
			<div class="flex flex-col items-center justify-center h-screen">
				<DownloadIcon class="h-14 mb-8" />
				<p>Installing extension. Do <b>NOT</b> close this window.</p>
			</div>
		{:else}
			<div>
				<div class="text-2xl font-bold mb-2">Clone Extension</div>
				<Input value={uiState.url} placeholder="Type the extension repo url" on:input={onSetUrl} />
			</div>
			<div class=" flex justify-end mt-2">
				<PrimaryButton text="Clone" disabled={uiState.disableCloneButton} on:click={onClone} />
			</div>
		{/if}
	</div>
</DialogFrame>
