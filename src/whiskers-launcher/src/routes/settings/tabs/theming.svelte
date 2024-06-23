<script lang="ts">
	import { getSettings, type Settings } from '$lib/settings/settings';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { createEventDispatcher } from 'svelte';
	import { save, open } from '@tauri-apps/api/dialog';
	import { downloadDir } from '@tauri-apps/api/path';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { WindowSizes } from '../../../utils';
	import { WebviewWindow } from '@tauri-apps/api/window';

	// =============================
	// Props
	// =============================
	export let settings: Settings;
	let dispatch = createEventDispatcher();

	// =============================
	// UI
	// =============================
	$: theme = settings.theme;

	// =============================
	// Events
	// =============================
	async function openStore() {
		new WebviewWindow('themes-store', {
			url: 'dialogs/themes-store',
			title: 'Themes Store',
			height: WindowSizes.Store.height,
			width: WindowSizes.Store.width,
			resizable: false,
			maximizable: false
		});

		const _ = await listen('refresh-theme', async () => {
			dispatch('updateTheme', (await getSettings()).theme);
		});
	}

	async function importTheme() {
		const path = await open({
			defaultPath: await downloadDir(),
			filters: [{ name: 'Whiskers Theme', extensions: ['wltheme'] }]
		});

		if (path) {
			invoke('get_theme_from_file', { path: path }).then((theme) => {
				dispatch('updateTheme', theme);
			});
		}
	}

	async function exportTheme() {
		const path = await save({
			defaultPath: `${await downloadDir()}/theme.wltheme`,
			filters: [{ name: 'Whiskers Theme', extensions: ['wltheme'] }]
		});

		if (path) {
			invoke('export_theme', { path: path });
		}
	}

	async function updateBackground(
		event: Event & { currentTarget: EventTarget & HTMLInputElement }
	) {
		let newTheme = settings.theme;
		newTheme.background = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateSecondary(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.secondary = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateTertiary(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.tertiary = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateAccent(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.accent = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateWarning(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.warning = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateDanger(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.danger = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateOnAccent(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.on_accent = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateOnDanger(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.on_danger = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateText(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.text = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}

	async function updateSubText(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		let newTheme = settings.theme;
		newTheme.sub_text = event.currentTarget.value;
		dispatch('updateTheme', newTheme);
	}
</script>

<div class=" flex">
	<SecondaryButton text="Themes Store" on:click={openStore} />
	<SecondaryButton text="Import" on:click={importTheme} />
	<SecondaryButton text="Export" on:click={exportTheme} />
</div>

<div class=" space-y-4">
	<div class=" card space-y-2">
		<div>
			<p class=" text-xl font-medium">Background Colors</p>
			<p class=" text-sub-text">
				The most used colors. They are mostly the background of the app, ui elements and border of
				elements.
			</p>
		</div>
		<div class=" flex gap-4">
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-background rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.background}
					on:input={updateBackground}
				/>
				<p class=" flex-grow text-center">Background</p>
			</div>
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-secondary rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.secondary}
					on:input={updateSecondary}
				/>
				<p class=" flex-grow text-center">Secondary</p>
			</div>
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-tertiary rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.tertiary}
					on:input={updateTertiary}
				/>
				<p class=" flex-grow text-center">Tertiary</p>
			</div>
		</div>
	</div>
	<div class=" card space-y-2">
		<div>
			<p class=" text-xl font-medium">Accent Colors</p>
			<p class=" text-sub-text">These colors are mostly used in buttons;</p>
		</div>
		<div class=" flex gap-4">
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-accent rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.accent}
					on:input={updateAccent}
				/>
				<p class=" flex-grow text-center">Accent</p>
			</div>
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-warning rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.warning}
					on:input={updateWarning}
				/>
				<p class=" flex-grow text-center">Warning</p>
			</div>
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-danger rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.danger}
					on:input={updateDanger}
				/>
				<p class=" flex-grow text-center">Danger</p>
			</div>
		</div>
		<div class=" flex gap-4">
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-on-accent rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.on_accent}
					on:input={updateOnAccent}
				/>
				<p class=" flex-grow text-center">On Accent</p>
			</div>
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-on-danger rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.on_danger}
					on:input={updateOnDanger}
				/>
				<p class=" flex-grow text-center">On Danger</p>
			</div>
		</div>
	</div>
	<div class=" card space-y-2">
		<div>
			<p class=" text-xl font-medium">Text Colors</p>
			<p class=" text-sub-text">The text colors.</p>
		</div>
		<div class=" flex gap-4">
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] class bg-text rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.text}
					on:input={updateText}
				/>
				<p class=" flex-grow text-center">Text</p>
			</div>
			<div class=" flex flex-col flex-grow">
				<input
					class=" h-[100px] bg-sub-text class rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
					type="color"
					value={theme.sub_text}
					on:input={updateSubText}
				/>
				<p class=" flex-grow text-center">Sub Text</p>
			</div>
		</div>
	</div>
</div>

<style scoped>
	input[type='color']::-webkit-color-swatch {
		border: none;
	}
</style>
