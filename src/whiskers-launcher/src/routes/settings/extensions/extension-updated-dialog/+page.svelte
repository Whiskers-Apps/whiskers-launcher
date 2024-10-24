<script lang="ts">
	import Dialog from '$lib/components/dialog.svelte';
	import { onCloseDialog } from './extension-updated-dialog-vm';
</script>

<Dialog
	title="Extension Updated"
	description="Extension Updated Successfully"
	actionText="OK"
	on:click={onCloseDialog}
/>
