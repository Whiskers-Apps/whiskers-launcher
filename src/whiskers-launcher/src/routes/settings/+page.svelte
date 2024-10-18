<script lang="ts">
	import { onMount } from 'svelte';
	import MainFrame from './main-frame.svelte';
	import {
		state,
		init,
		onSetShowRecentApps,
		onSetAutoStart,
		onSetFirstKey,
		onSetSecondKey,
		onSetThirdKey
	} from './general-settings-page-vm';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import Warning from '$lib/icons/warning.svg?component';
	import Select from '$lib/components/select.svelte';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class=" space-y-8">
			<div>
				{#if !uiState.isWayland}
					<p class=" text-xl font-medium">Shortcut Keys</p>
					<p class=" text-sub-text">The shortcut to spawn the search window.</p>
					<div class=" space-y-2">
						<div>
							<p class="">First Key</p>
							<Select
								values={uiState.firstKeyOptions}
								selectedValue={uiState.settings.first_key}
								on:selection={onSetFirstKey}
							/>
						</div>
						<div>
							<p class="">Second Key</p>
							<Select
								values={uiState.secondKeyOptions}
								selectedValue={uiState.settings.second_key ?? '-'}
								on:selection={onSetSecondKey}
							/>
						</div>
						<div>
							<p class="">Third Key</p>
							<Select
								values={uiState.thirdKeyOptions}
								selectedValue={uiState.settings.third_key}
								on:selection={onSetThirdKey}
							/>
						</div>
					</div>
					{#if uiState.showShortcutWarnings}
						<div class=" flex items-center text-warning">
							<Warning class="w-6 h-6 mr-2" />
							<p>
								The launcher needs a restart to apply this setting. Please restart using the system
								tray app
							</p>
						</div>
					{/if}
				{:else}
					<div class=" flex items-center text-warning">
						<Warning class="w-6 h-6 mr-2" />
						<p>
							Wayland detected. Please add the shortcut to run "whiskers-launcher" manually from
							your DE/WM settings.
						</p>
					</div>
				{/if}
			</div>

			{#if uiState.os !== 'windows'}
				<ToggleSetting
					title="Recent Apps"
					description="When enabled, it shows the most recent opened apps when opening the launcher"
					toggled={uiState.settings.show_recent_apps}
					on:toggle={onSetShowRecentApps}
				/>
			{/if}

			<ToggleSetting
				title="Auto Start"
				description="When enabled, it auto starts the app at login"
				toggled={uiState.settings.auto_start}
				on:toggle={onSetAutoStart}
			/>
		</div>
	{/if}
</MainFrame>
