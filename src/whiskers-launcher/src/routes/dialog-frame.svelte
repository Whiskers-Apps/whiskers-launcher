<script lang="ts">
	import { onMount } from 'svelte';
	import { init, state } from './dialog-frame-vm';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

{#if !uiState.loading}
	{@html uiState.css}
    <div class="bg-background text-text">
        <slot></slot>
    </div>
{/if}
