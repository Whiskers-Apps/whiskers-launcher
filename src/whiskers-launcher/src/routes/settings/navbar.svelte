<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	// ========================================
	// UI
	// ========================================

	let generalButton: HTMLButtonElement;
	let searchBoxButton: HTMLButtonElement;
	let searchResultsButton: HTMLButtonElement;
	let searchEnginesButton: HTMLButtonElement;
	let themingButton: HTMLButtonElement;
	let extensionButton: HTMLButtonElement;
	let aboutButton: HTMLButtonElement;

	// ========================================
	// Events
	// ========================================

	const dispatch = createEventDispatcher();

	function selectTab(tab: number) {
		dispatch('tabSelected', tab);

		generalButton.classList.remove('text-accent');
		searchBoxButton.classList.remove('text-accent');
		searchResultsButton.classList.remove('text-accent');
		searchEnginesButton.classList.remove('text-accent');
		themingButton.classList.remove('text-accent');
		extensionButton.classList.remove('text-accent');
		aboutButton.classList.remove('text-accent');

		switch (tab) {
			case 0:
				generalButton.classList.add('text-accent');
				break;
			case 1:
				searchBoxButton.classList.add('text-accent');
				break;
			case 2:
				searchResultsButton.classList.add('text-accent');
				break;
			case 3:
				searchEnginesButton.classList.add('text-accent');
				break;
			case 4:
				themingButton.classList.add('text-accent');
				break;
			case 5:
				extensionButton.classList.add('text-accent');
				break;
			case 6:
				aboutButton.classList.add('text-accent');
				break;
		}
	}
</script>

<div class="p-4 items-start space-y-4 max-w-[250px] overflow-auto">
	<button
		bind:this={generalButton}
		class="text-xl hover-text-accent text-accent w-full text-start"
		on:click={() => {
			selectTab(0);
		}}>General</button
	>
	<button
		bind:this={searchBoxButton}
		class="text-xl hover-text-accent w-full text-start"
		on:click={() => {
			selectTab(1);
		}}>Search Box</button
	>

	<button
		bind:this={searchResultsButton}
		class="text-xl hover-text-accent w-full text-start"
		on:click={() => {
			selectTab(2);
		}}>Search Results</button
	>

	<button
		bind:this={searchEnginesButton}
		class="text-xl hover-text-accent w-full text-start"
		on:click={() => {
			selectTab(3);
		}}>Search Engines</button
	>

	<button
		bind:this={themingButton}
		class="text-xl hover-text-accent w-full text-start"
		on:click={() => {
			selectTab(4);
		}}>Theming</button
	>

	<button
		bind:this={extensionButton}
		class="text-xl hover-text-accent w-full text-start"
		on:click={() => {
			selectTab(5);
		}}>Extensions</button
	>

	<button
		bind:this={aboutButton}
		class="text-xl hover-text-accent w-full text-start"
		on:click={() => {
			selectTab(6);
		}}>About</button
	>
</div>
