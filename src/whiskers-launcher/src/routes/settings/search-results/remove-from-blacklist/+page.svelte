<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/dialog.svelte';
	import DialogFrame from '../../../dialog-frame.svelte';
	import { onRemoveAppFromBlacklist } from './remove-from-blacklist-dialog-vm';

	let id = decodeURIComponent($page.url.searchParams.get('id')!!);
</script>

	<Dialog
		title="Remove From Blacklist"
		description="Are you sure you want to remove this from the blacklist?"
		actionText="Remove"
		on:click={() => onRemoveAppFromBlacklist(id)}
	/>
