<script lang="ts">
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import PlusIcon from '$lib/icons/plus.svg?component';
	import MinusIcon from '$lib/icons/minus.svg?component';
	import QuestionIcon from '$lib/icons/question.svg?component';
	import { type App, type Settings } from '$lib/settings/settings';
	import { invoke } from '@tauri-apps/api';
	import { createEventDispatcher, onMount } from 'svelte';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { listen } from '@tauri-apps/api/event';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	// ================================
	// Props
	// ================================
	export let settings: Settings;
	let dispatch = createEventDispatcher();

	// ================================
	// UI
	// ================================
	$: highlightBackground = settings.highlight_selected_background;
	$: showAltHint = settings.show_alt_hint;
	$: resultsCount = settings.results_count;
	let blacklist_apps: App[] = [];

	onMount(async () => {
		blacklist_apps = await invoke('get_blacklisted_apps');
	});

	async function updateHighlightBackground(event: CustomEvent<boolean>) {
		dispatch('updateHighlightBackground', event.detail);
	}

	async function updateShowAltHint(event: CustomEvent<boolean>) {
		dispatch('updateShowAltHint', event.detail);
	}

	async function updateResultsCount(event: CustomEvent<number>) {
		dispatch('updateResultsCount', event.detail);
	}

	async function openAddToBlacklistDialog() {
		new WebviewWindow('add-to-blacklist', {
			url: 'dialogs/add-to-blacklist',
			title: 'Add To Blacklist',
			resizable: false,
			width: 800,
			height: 700,
			center: true
		});

		const unlisten = await listen('refresh-blacklist', async (_) => {
			dispatch('refreshBlacklist');
			blacklist_apps = await invoke('get_blacklisted_apps');
			unlisten();
		});
	}

	async function openRemoveDialog(id: string) {
		new WebviewWindow('remove-from-blacklist', {
			url: `dialogs/remove-from-blacklist/?id=${encodeURIComponent(id)}`,
			title: 'Remove from Blacklist',
			resizable: false,
			width: 800,
			height: 200,
			center: true
		});

		const unlisten = await listen('refresh-blacklist', async (_) => {
			dispatch('refreshBlacklist');
			blacklist_apps = await invoke('get_blacklisted_apps');
			unlisten();
		});
	}
</script>

<div class=" space-y-4">
	<ToggleSetting
		title="Highlight Background"
		description="When enabled, it changes the background color of the selected result."
		toggled={highlightBackground}
		on:toggle={updateHighlightBackground}
	/>

	<ToggleSetting
		title="Show Alt Hint"
		description="When enabled, it shows the 'alt + key' hint in the results."
		toggled={showAltHint}
		on:toggle={updateShowAltHint}
	/>

	<SliderSetting
		title={`Results Count (${resultsCount})`}
		description="The amount of results to show."
		min={2}
		max={9}
		step={1}
		value={resultsCount}
		on:slide={updateResultsCount}
	/>

	<div class="card">
		<div class=" flex items-center gap-4">
			<p class=" text-xl font-medium flex-grow">Blacklist</p>
			<button class=" hover-bg-tertiary p-1 rounded-full" on:click={openAddToBlacklistDialog}>
				<PlusIcon class="w-6 h-6 text-accent " />
			</button>
		</div>
		{#if blacklist_apps.length === 0}
			<p class=" text-sub-text">Blacklist is empty. Click on the add button to add apps to it.</p>
		{/if}
		{#each blacklist_apps as app}
			<div class=" flex gap-4 pt-2 pl-2 pb-2 items-center">
				{#if app.icon !== null}
					<img class=" h-6 w-6" src={convertFileSrc(app.icon)} alt="App Icon" />
				{:else}
					<QuestionIcon class=" h-6 w-6" />
				{/if}
				<p class=" flex-grow text-start">
					{app.title}
				</p>
				<button
					class=" hover-bg-tertiary p-1 rounded-full"
					on:click={() => openRemoveDialog(app.id)}
				>
					<MinusIcon class="w-6 h-6 text-accent " />
				</button>
			</div>
		{/each}
	</div>
</div>
