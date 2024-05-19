<script lang="ts">
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
	import { createEventDispatcher, onMount } from 'svelte';

	// =================================
	// Props
	// =================================
	export let title: string;
	export let description: string;
	export let actionText: string;
	let dispatch = createEventDispatcher();

	let settings: Settings | null = null;
	let css = '';

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
	});
</script>

{#if settings !== null}
	{@html css}
	<div
		class=" p-4 bg-background text-text overflow-auto flex flex-col justify-center h-screen w-full"
	>
		<p class=" text-2xl font-bold">{title}</p>
		<p class=" text-sub-text">{description}</p>
		<div class=" flex justify-end mt-4">
			<PrimaryButton text={actionText} on:click={() => dispatch('click')} />
		</div>
	</div>
{/if}
