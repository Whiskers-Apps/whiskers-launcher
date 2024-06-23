<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Toggle from './toggle.svelte';

	export let title: string;
	export let description: string;
	export let toggled: boolean;

	let dispatch = createEventDispatcher();

	function onToggle(event: CustomEvent<boolean>) {
		dispatch('toggle', event.detail);
	}
</script>

<div class="card flex items-start gap-4">
	<div class=" flex-grow">
		<p class=" text-xl font-medium">{title}</p>
		<p class=" text-sub-text">{description}</p>
	</div>
	<div>
		<Toggle {toggled} on:toggle={onToggle} />
	</div>
</div>
