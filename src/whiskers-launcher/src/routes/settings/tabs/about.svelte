<script lang="ts">
	import type { Settings } from '$lib/settings/settings';
	import { getVersion } from '@tauri-apps/api/app';
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';

	// ====================================
	// Props
	// ====================================
	export let settings: Settings;
	console.log(settings);
	

	// ====================================
	// UI Elements
	// ====================================
	let appVersion = '';
	let extensionsCount = 0;
	const GITHUB_LINK = 'https://github.com/Whiskers-Apps/whiskers-launcher/';

	// ====================================
	// Events
	// ====================================
	onMount(async () => {
		appVersion = await getVersion();
	});

	function openLink() {
		open(GITHUB_LINK);
	}
</script>

<div class=" space-y-4">
	<div class="card">
		<p class=" text-xl font-medium">Version</p>
		<p class=" text-sub-text">{appVersion}</p>
	</div>

	<div class="card">
		<p class=" text-xl font-medium">Extensions</p>
		<p class=" text-sub-text">{extensionsCount}</p>
	</div>

	<div class="card">
		<p class=" text-xl font-medium">Source Code</p>
		<button class=" underline hover-text-accent text-sub-text" on:click={openLink}>{GITHUB_LINK}</button>
	</div>
</div>
