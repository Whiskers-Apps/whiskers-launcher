<script lang="ts">
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import CheckIcon from '$lib/icons/check.svg?component';
	import QuestionIcon from '$lib/icons/question.svg?component';
	import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { type App } from '$lib/settings/settings';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { emit } from '@tauri-apps/api/event';

	let settings: Settings | null = null;
	let css = '';
	let whiteListedApps: App[] = [];
	let selectedApps: string[] = [];

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
		whiteListedApps = await invoke('get_whitelisted_apps');
	});

	function toggleApp(app: App) {
		if (selectedApps.includes(app.id)) {
			selectedApps = selectedApps.filter((appId) => appId !== app.id);
		} else {
			selectedApps.push(app.id);
		}

		selectedApps = [...selectedApps];
	}

	async function addApps() {
		for (const id in selectedApps) {
			console.log(id)
			await invoke('add_to_blacklist', { id: selectedApps[id] });
		}

		await emit('refresh-blacklist');

		appWindow.close();
	}
</script>

{@html css}
<div class=" h-screen w-full bg-background text-text p-4 overflow-auto flex flex-col">
	<div class=" flex-grow overflow-auto">
		{#each whiteListedApps as app}
			<button
				class=" flex items-center p-2 gap-4 w-full hover-bg-secondary rounded-md"
				on:click={() => toggleApp(app)}
			>
				{#if app.icon !== null}
					<img class=" h-6 w-6" src={convertFileSrc(app.icon)} alt="App Icon" />
				{:else}
					<QuestionIcon class=" h-6 w-6" />
				{/if}
				<p class=" flex-grow text-start">
					{app.title}
				</p>
				{#if selectedApps.includes(app.id)}
					<CheckIcon class=" h-6 w-6 text-accent" />
				{/if}
			</button>
		{/each}
	</div>
	<div class=" flex justify-end">
		<PrimaryButton text="Add" disabled={selectedApps.length === 0} on:click={addApps} />
	</div>
</div>
