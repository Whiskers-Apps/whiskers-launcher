<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import TextArea from './text-area.svelte';

	export let title;
	export let description;
	export let value;
	export let placeholder = '';

	let dispatch = createEventDispatcher();

	function onInput(event: CustomEvent<string>) {
		dispatch('input', event.detail);
	}
</script>

<div class="space-y-2">
	<div>
		<p class=" text-xl font-medium">{title}</p>
		<p class=" text-sub-text">{description}</p>
	</div>
	<TextArea {value} {placeholder} on:input={onInput} />
</div>
