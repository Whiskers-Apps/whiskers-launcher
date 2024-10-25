<script lang="ts">
	import type { SelectValue } from './classes';
	import ChevronDown from '$lib/icons/chevron-down.svg?component';
	import ChevronUp from '$lib/icons/chevron-up.svg?component';
	import { createEventDispatcher } from 'svelte';

	export let values: SelectValue[];
	export let selectedValue: string;
	let showOptions: boolean = false;
	let dispatch = createEventDispatcher();

	$: value = values.filter((v) => v.id === selectedValue)[0];

	function toggleShowOptions() {
		showOptions = !showOptions;
	}

	function selectValue(value: SelectValue) {
		dispatch('selection', value);
		showOptions = false;
	}
</script>

<div class="select">
	<button class=" w-full items-center flex gap-4 p-2" on:click={toggleShowOptions}>
		<p class=" flex-grow text-start">{value.value}</p>
		{#if showOptions}
			<ChevronUp class=" h-4 w-4" />
		{:else}
			<ChevronDown class=" h-4 w-4" />
		{/if}
	</button>
	{#if showOptions}
		<div class=" v-divider"></div>
		<div class=" flex flex-col">
			{#each values as value}
				<button
					class={`w-full flex items-start p-2 hover-bg-tertiary ${selectedValue === value.id ? ' bg-secondary' : ''}`}
					on:click={() => {
						selectValue(value);
					}}>{value.value}</button
				>
			{/each}
		</div>
	{/if}
</div>

<style>
	.select {
		background-color: var(--secondary);
		border: 1px solid var(--tertiary);
		border-radius: 14px;
		width: 100%;
	}
</style>
