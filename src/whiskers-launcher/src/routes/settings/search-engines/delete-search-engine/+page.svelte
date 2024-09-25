<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/dialog.svelte';
	import { onMount } from 'svelte';
	import { onDelete } from './delete-search-engine-dialog-vm';
	let id = 0;

	onMount(() => {
		id = +$page.url.searchParams.get('id')!!;
	});
</script>

<Dialog
	title="Delete Search Engine"
	description="Are you sure you want to delete the search engine?"
	actionText="Delete"
	on:click={() => onDelete(id)}
/>
