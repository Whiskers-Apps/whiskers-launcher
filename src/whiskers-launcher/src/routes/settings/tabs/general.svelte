<script lang="ts">
	import type { SelectValue } from '$lib/components/classes';
	import Select from '$lib/components/select.svelte';
	import {
		LAUNCH_FIRST_KEY_OPTIONS as FIRST_KEY_OPTIONS,
		LAUNCH_THIRD_KEY_OPTIONS as THIRD_KEY_OPTIONS,
		LAUNCH_SECOND_KEY_OPTIONS as SECOND_KEY_OPTIONS,
		type Settings
	} from '$lib/settings/settings';
	import Warning from '$lib/icons/warning.svg?component';
	import { createEventDispatcher, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import SliderSetting from '$lib/components/slider-setting.svelte';

	// ==============================
	// Props
	// ==============================
	export let settings: Settings;
	let dispatch = createEventDispatcher();

	// ==============================
	// UI Elements
	// ==============================
	$: firstKey = settings.first_key;
	$: secondKey = settings.second_key;
	$: thirdKey = settings.third_key;
	$: scaling = settings.scaling;
	$: autoStart = settings.auto_start;
	$: showRecentApps = settings.show_recent_apps;

	let showShortcutWarning = false;
	let shortcutSettingsCard: HTMLDivElement;
	let isWayland = false;

	// ==============================
	// Events
	// ==============================
	onMount(async () => {
		isWayland = await invoke('is_wayland');
	});

	async function updateFirstKey(value: CustomEvent<SelectValue>) {
		dispatch('updateFirstKey', value.detail.id);
		warnShortcutChange();
	}

	async function updateSecondKey(value: CustomEvent<SelectValue>) {
		dispatch('updateSecondKey', value.detail.id === '-' ? null : value.detail.id);
		warnShortcutChange();
	}

	async function updateThirdKey(value: CustomEvent<SelectValue>) {
		dispatch('updateThirdKey', value.detail.id);
		warnShortcutChange();
	}

	async function updateScaling(value: CustomEvent<number>) {
		dispatch('updateScaling', value.detail);
	}

	async function updateAutoStart(value: CustomEvent<boolean>) {
		dispatch('updateAutoStart', value.detail);
	}

	async function updateShowRecentApps(value: CustomEvent<boolean>) {
		dispatch('updateShowRecentApps', value.detail);
	}

	function warnShortcutChange() {
		showShortcutWarning = true;
		shortcutSettingsCard.classList.add('warning');
	}
</script>

<div class=" space-y-4">
	<div class="card" bind:this={shortcutSettingsCard}>
		{#if !isWayland}
			<p class=" text-xl font-medium">Shortcut Keys</p>
			<p class=" text-sub-text">The shortcut to spawn the search window.</p>
			<div class=" space-y-2">
				<div>
					<p class="">First Key</p>
					<Select
						values={FIRST_KEY_OPTIONS}
						selectedValue={firstKey}
						on:selection={updateFirstKey}
					/>
				</div>
				<div>
					<p class="">Second Key</p>
					<Select
						values={SECOND_KEY_OPTIONS}
						selectedValue={secondKey ?? '-'}
						on:selection={updateSecondKey}
					/>
				</div>
				<div>
					<p class="">Third Key</p>
					<Select
						values={THIRD_KEY_OPTIONS}
						selectedValue={thirdKey}
						on:selection={updateThirdKey}
					/>
				</div>
			</div>
			{#if showShortcutWarning}
				<div class=" flex items-center text-warning">
					<Warning class="w-6 h-6 mr-2" />
					<p>
						The launcher needs a restart to apply this setting. Please restart using the system tray
						app
					</p>
				</div>
			{/if}
		{:else}
			<div class=" flex items-center text-warning">
				<Warning class="w-6 h-6 mr-2" />
				<p>
					Wayland detected. Please add the shortcut to run "whiskers-launcher" manually from your
					DE/WM settings.
				</p>
			</div>
		{/if}
	</div>

	<SliderSetting
		title={`Scaling (${scaling.toFixed(2)})`}
		description="This scaling is applied in the search window"
		min={0.5}
		max={2}
		step={0.1}
		value={scaling}
		on:slide={updateScaling}
	/>

	<ToggleSetting
		title="Auto Start"
		description="When enabled, it auto starts the app at login"
		toggled={autoStart}
		on:toggle={updateAutoStart}
	/>

	<ToggleSetting
		title="Recent Apps"
		description="When enabled, it shows the most recent opened apps when opening the launcher"
		toggled={showRecentApps}
		on:toggle={updateShowRecentApps}
	/>
</div>
