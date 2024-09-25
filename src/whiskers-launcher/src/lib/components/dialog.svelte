<script lang="ts">
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import { createEventDispatcher, onMount } from 'svelte';
	import DialogFrame from '../../routes/dialog-frame.svelte';

	export let title: string;
	export let description: string;
	export let actionText: string;
	let dispatch = createEventDispatcher();

	function onClick() {
		dispatch('click');
	}
</script>

<DialogFrame>
	<div class=" p-4 overflow-auto flex flex-col justify-center h-screen w-full">
		<p class=" text-2xl font-bold">{title}</p>
		<p class=" text-sub-text">{description}</p>
		<div class=" flex justify-end mt-4">
			<PrimaryButton text={actionText} on:click={onClick} />
		</div>
	</div>
</DialogFrame>
