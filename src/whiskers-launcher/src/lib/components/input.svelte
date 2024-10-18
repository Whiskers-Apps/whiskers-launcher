<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let value = '';
	export let placeholder = '';

	let dispatch = createEventDispatcher();

	function handleInput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		dispatch('input', event.currentTarget.value);
	}
</script>

<input class=" input" bind:value={value} on:input={handleInput} {placeholder} />

<style>
	.input {
		background-color: var(--secondary);
		border: 1px solid var(--tertiary);
		padding: 8px;
		border-radius: 14px;
		width: 100%;
	}

	.input:focus {
		outline: 1px solid var(--accent);
	}

	.input::placeholder {
		color: var(--sub-text);
	}
</style>
