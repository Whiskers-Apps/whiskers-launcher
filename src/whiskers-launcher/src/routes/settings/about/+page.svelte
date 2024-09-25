<script lang="ts">
	import MainFrame from '../main-frame.svelte';
	import logo from '$lib/images/whiskers-launcher.webp';
	import buyMeACoffeeLogo from "$lib/images/bmc-button.webp";
	import { onMount } from 'svelte';
	import { GITHUB_REPO_URL, state, init, onOpenGitHubRepo, onOpenBuyMeACoffeePage } from './about-page-vm';

	$: uiState = $state;

	onMount(async () => {
		init()
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class=" space-y-4">
			<div class=" flex justify-center">
				<img src={logo} alt="whiskers launcher logo" width="200" />
			</div>
			<div>
				<p class=" text-xl font-medium">Version</p>
				<p class=" text-sub-text">{uiState.appVersion}</p>
			</div>

			<div>
				<p class=" text-xl font-medium">Extensions</p>
				<p class=" text-sub-text">{uiState.extensionsCount}</p>
			</div>

			<div>
				<p class=" text-xl font-medium">Source Code</p>
				<button
					class=" underline hover-text-accent text-sub-text"
					on:click={onOpenGitHubRepo}
				>
					{GITHUB_REPO_URL}
				</button>
			</div>

			<div>
				<p class=" text-xl font-medium">Donate</p>
				<button
					class=" underline hover-text-accent text-sub-text "
					on:click={onOpenBuyMeACoffeePage}
				>
					<img src={buyMeACoffeeLogo} alt="buy me a coffee logo" class="h-[60px]">
				</button>
			</div>
		</div>
	{/if}
</MainFrame>
