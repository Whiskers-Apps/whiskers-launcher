<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let text: string;
	export let disabled = false;

	let dispatch = createEventDispatcher();

	function onClick(event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
		dispatch('click');
	}
</script>

<button class="pr-button p-2 pr-3 pl-3 text-on-accent rounded-full" on:click={onClick} {disabled}>
	{text}
</button>

<style>
	.pr-button {
		background-color: var(--accent);
	}

	.pr-button:hover:enabled {
		filter: brightness(0.9);
	}

	.pr-button:disabled {
		background-color: var(--secondary);
		color: var(--text);
		outline: 1px solid var(--tertiary);
	}
</style>
