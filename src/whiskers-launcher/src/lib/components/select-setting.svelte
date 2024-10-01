<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Input from './input.svelte';
	import Select from './select.svelte';
	import type { SelectValue } from './classes';

	export let title: string;
	export let description: string;
	export let values: SelectValue[];
	export let selectedValue: string;

	let dispatch = createEventDispatcher();

	function onSelection(event: CustomEvent<SelectValue>) {
		dispatch('selection', event.detail);
	}
</script>

<div class="space-y-2">
	<div>
		<p class=" text-xl font-medium">{title}</p>
		<p class=" text-sub-text">{description}</p>
	</div>
	<Select {values} {selectedValue} on:selection={onSelection} />
</div>
