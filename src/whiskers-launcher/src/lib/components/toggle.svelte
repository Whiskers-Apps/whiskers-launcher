<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let toggled: boolean;

	let dispatch = createEventDispatcher();

	function handleInput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		dispatch('toggle', event.currentTarget.checked);
	}
</script>

<label class="switch ml-2">
	<input type="checkbox" checked={toggled} on:input={handleInput} />
	<span class="slider round"></span>
</label>

<style>
	.switch {
		position: relative;
		display: inline-block;
		width: 57px;
		height: 30px;
	}

	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: var(--background);
		border: 1px solid var(--accent);
	}

	.slider:before {
		position: absolute;
		content: '';
		height: 22px;
		width: 22px;
		left: 4px;
		bottom: 3px;
		background-color: var(--background);
		border: 2px solid var(--accent);
	}

	input:checked + .slider {
		background-color: var(--accent);
	}

	

	input:checked + .slider:before {
		-webkit-transform: translateX(26px);
		-ms-transform: translateX(26px);
		transform: translateX(26px);
	}

	/* Rounded sliders */
	.slider.round {
		border-radius: 34px;
	}

	.slider.round:before {
		border-radius: 50%;
	}
</style>
