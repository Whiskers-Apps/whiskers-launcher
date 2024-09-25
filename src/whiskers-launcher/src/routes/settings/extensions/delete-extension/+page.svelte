<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/dialog.svelte';
	import { onRemoveApp } from './delete-extension-dialog-vm';
	
	let id = decodeURIComponent($page.url.searchParams.get('id')!!);
</script>

<Dialog
	title="Delete Extension"
	description="Are you sure you want to delete this extension?"
	actionText="Delete"
	on:click={() => onRemoveApp(id)}
/>