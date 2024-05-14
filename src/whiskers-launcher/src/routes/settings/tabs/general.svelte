<script lang="ts">
	import type { SelectValue } from '$lib/components/classes';
	import Select from '$lib/components/select.svelte';
	import {
		LAUNCH_FIRST_KEY_OPTIONS as FIRST_KEY_OPTIONS,
		LAUNCH_THIRD_KEY_OPTIONS as THIRD_KEY_OPTIONS,
		LAUNCH_SECOND_KEY_OPTIONS as SECOND_KEY_OPTIONS,
		type Settings
	} from '$lib/settings/settings';

	// ==============================
	// Props
	// ==============================
	export let settings: Settings;

	// ==============================
	// UI Elements
	// ==============================
	let firstKey = settings.first_key;
	let secondKey = settings.second_key;
	let thirdKey = settings.third_key;

	function updateFirstKey(value: CustomEvent<SelectValue>) {
		firstKey = value.detail.id;
	}

	function updateSecondKey(value: CustomEvent<SelectValue>) {
		secondKey = value.detail.id;
	}

	function updateThirdKey(value: CustomEvent<SelectValue>) {
		thirdKey = value.detail.id;
	}
</script>

<div class=" space-y-4">
	<div class="card">
		<p class=" text-xl font-medium">Shortcut Keys</p>
		<p class=" text-sub-text">The shortcut to spawn the search window.</p>
		<div class=" space-y-2">
			<div>
				<p class="">First Key</p>
				<Select values={FIRST_KEY_OPTIONS} selectedValue={firstKey} on:selection={updateFirstKey} />
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
				<Select values={THIRD_KEY_OPTIONS} selectedValue={thirdKey} on:selection={updateThirdKey} />
			</div>
		</div>
	</div>
</div>
