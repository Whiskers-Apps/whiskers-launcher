<script lang="ts">
	import logo from "$lib/images/whiskers-launcher.webp";
	import type { Extension, Settings } from '$lib/settings/settings';
	import { invoke } from '@tauri-apps/api';
	import { getVersion } from '@tauri-apps/api/app';
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';

	// ====================================
	// Props
	// ====================================
	export let settings: Settings;

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
		let extensions: Extension[] = await invoke("get_extensions")
		extensionsCount = extensions.length;
	});

	function openLink() {
		open(GITHUB_LINK);
	}
</script>

<div class=" space-y-4">
	<div class=" flex justify-center">
		<img src={logo} alt="whiskers launcher logo" width="200">
	</div>
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
