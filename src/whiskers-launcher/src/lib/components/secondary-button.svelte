<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let text: string;
	export let disabled = false;
	export let alignStart = false;

	let dispatch = createEventDispatcher();

	function onClick(event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
		dispatch('click');
	}
</script>

<button class={`sec-button p-2 pr-3 pl-3 text-on-accent rounded-full ${alignStart ? 'text-start' : ''}`} on:click={onClick} {disabled}>
	{text}
</button>

<style>
	.sec-button {
        color: var(--accent);
	}

	.sec-button:hover:enabled {
		font-weight: bold;
	}

	.sec-button:disabled {
		color: var(--text);
	}
</style>
