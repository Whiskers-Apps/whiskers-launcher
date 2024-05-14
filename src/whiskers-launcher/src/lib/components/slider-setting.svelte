<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Slider from './slider.svelte';

	export let title: string;
	export let description: string;
	export let min: number;
	export let max: number;
	export let step: number;
	export let value: number;

	let dispatch = createEventDispatcher();

	function onSlide(event: CustomEvent<number>) {
		dispatch('slide', event.detail);
	}
</script>

<div>
	<p class=" text-xl font-medium">{title}</p>
	<p class=" text-sub-text">{description}</p>
</div>
<Slider {min} {max} {step} {value} on:slide={onSlide} />
