<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Toggle from './toggle.svelte';

	export let text = '';
    export let placeholder = '';

	let dispatch = createEventDispatcher();

	function handleInput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		dispatch('input', event.currentTarget.value);
	}
</script>

<input class=" input" bind:value={text} on:input={handleInput} placeholder={placeholder} />