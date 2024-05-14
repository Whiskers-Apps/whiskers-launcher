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
		width: 50px;
		height: 26px;
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
		border: 2px solid var(--accent);
	}

	.slider:before {
		position: absolute;
		content: '';
		height: 20px;
		width: 20px;
		left: 0px;
		bottom: 1px;
		background-color: var(--background);
		border: 2px solid var(--accent);
	}

	input:checked + .slider {
		background-color: var(--accent);
	}

	

	input:checked + .slider:before {
		-webkit-transform: translateX(28px);
		-ms-transform: translateX(28px);
		transform: translateX(28px);
	}

	/* Rounded sliders */
	.slider.round {
		border-radius: 34px;
	}

	.slider.round:before {
		border-radius: 50%;
	}
</style>
