<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let min: number;
	export let max: number;
	export let step: number;
	export let value: number;

	let dispatch = createEventDispatcher();

	function handleSlide(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		dispatch('slide', +event.currentTarget.value);
	}
</script>

<div class="w-full flex overflow-hidden rounded-full border-accent">
	<input type="range" class=" flex-grow" {min} {max} {step} {value} on:input={handleSlide} />
</div>

<style>
	input[type='range'] {
		-webkit-appearance: none;
		appearance: none;
		background: transparent;
		cursor: pointer;
		width: 15rem;
	}

	input[type='range']:focus {
		outline: none;
	}

	input[type='range'] {
		margin: auto;
		-webkit-appearance: none;
		appearance: none;
		position: relative;
		overflow: hidden;
		height: 30px;
		width: 200px;
		cursor: pointer;
		border-radius: 0;
	}

	::-webkit-slider-runnable-track {
		background: var(--background);
		border-radius: 999px;
	}

	::-webkit-slider-thumb {
		-webkit-appearance: none;
		width: 30px;
		height: 30px;
		background: var(--background);
		box-shadow: -2000px 0 0 1985px var(--accent);
		border: 3px solid var(--accent);
		border-radius: 999px;
	}

	::-ms-fill-lower {
		background: var(--accent);
	}

	::-ms-ticks-after {
		display: none;
	}

	::-ms-ticks-before {
		display: none;
	}

	::-ms-track {
		background: var(--accent);
		color: transparent;
		height: 40px;
		border: none;
	}

	::-ms-tooltip {
		display: none;
	}
</style>
