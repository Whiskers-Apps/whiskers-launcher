<script lang="ts">
	import QuestionIcon from '$lib/icons/question.svg?component';
	import SettingsIcon from '$lib/icons/settings.svg?component';
	import SearchIcon from '$lib/icons/search.svg?component';
	import { getSettings, getThemeCss, type Settings, type WLResult } from '$lib/settings/settings';
	import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	// ===========================
	// UI
	// ===========================
	let settings: Settings | null = null;
	let css = '';

	let searchText = '';
	let results: WLResult[] = [];
	let displayedResults: WLResult[] = [];
	$: selectedIndex = 0;
	let resultOffset = 0;

	// ===========================
	// UI Events
	// ===========================
	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
	});

	appWindow.setResizable(false);

	window.addEventListener('keydown', (event) => {
		switch (event.key) {
			case 'Escape': {
				appWindow.close();
				break;
			}
			case 'ArrowUp': {
				event.preventDefault();
				goToPreviousResult();
				break;
			}
			case 'ArrowDown': {
				event.preventDefault();
				goToNextResult();
				break;
			}

			case 'Enter': {
				runAction();
				break;
			}
		}

		if (event.ctrlKey && event.key === 's') {
			openSettings();
			appWindow.close();
		}

		if (event.altKey && ['1', '2', '3', '4', '5', '6', '7', '8'].includes(event.key)) {
			selectAltResult(+event.key);
		}
	});

	async function openSettings() {
		invoke('open_settings_window');

		setTimeout(() => {
			appWindow.close();
		}, 200);
	}

	async function search(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		searchText = event.currentTarget.value;

		results = await invoke('get_results', { text: searchText });

		selectedIndex = 0;
		resultOffset = 0;

		cutResults();
	}

	async function cutResults() {
		displayedResults = [...results.slice(resultOffset, resultOffset + settings!!.results_count)];
	}

	async function goToPreviousResult() {
		if (selectedIndex > 0) {
			selectedIndex--;
			return;
		}

		if (resultOffset - 1 > 0) {
			resultOffset--;
			cutResults();
			return;
		}

		if (resultOffset === 0) {
			if (results.length < settings!!.results_count) {
				selectedIndex = results.length - 1;
				return;
			}
		}

		resultOffset = results.length - settings!!.results_count;
		selectedIndex = settings!!.results_count - 1;
		cutResults();
	}

	async function goToNextResult() {
		if (selectedIndex < displayedResults.length - 1) {
			selectedIndex++;
			return;
		}

		if (resultOffset + selectedIndex < results.length - 1) {
			resultOffset++;
			cutResults();
			return;
		}

		if (results.length < settings!!.results_count) {
			if (selectedIndex + 1 === results.length) {
				selectedIndex = 0;
				return;
			}
		}

		resultOffset = 0;
		selectedIndex = 0;
		cutResults();
	}

	async function selectAltResult(index: number){
		if(index - 1 > displayedResults.length){
			return;
		}

		selectedIndex = index - 1;
		runAction();
	}

	async function runAction(){
		invoke("run_action", {result: displayedResults[selectedIndex]})
	}
</script>

<div>
	{#if settings !== null}
		{@html css}
		<div class="h-screen overflow-hidden flex justify-center text-text pt-16">
			<div class=" w-[800px] h-fit search-round search-border overflow-hidden">
				<div class=" flex bg-background p-3 gap-2">
					{#if settings.show_search_icon}
						<SearchIcon class=" search-icon-size text-accent" />
					{/if}

					<!-- svelte-ignore a11y-autofocus -->
					<input
						class=" w-full outline-none flex-grow bg-transparent"
						type="text"
						placeholder={settings.show_placeholder ? 'Search apps, extensions, web' : ''}
						value={searchText}
						on:input={search}
						autofocus
					/>

					{#if settings.show_settings_icon}
						<button class=" hover-bg-tertiary rounded-full" on:click={openSettings}>
							<SettingsIcon class=" search-icon-size text-accent" />
						</button>
					{/if}
				</div>

				<div class=" bg-background">
					{#each displayedResults as result, index}
						<div class={`flex items-center ${index === selectedIndex ? 'highlight-result' : ''}`}>
							{#if result.result_type === 'Text'}
								<div class="w-full flex gap-4 p-3 items-center">
									{#if result.text?.icon !== null}
										<img class=" icon-size" src={convertFileSrc(result.text?.icon ?? '')} alt="" />
									{:else}
										<QuestionIcon class=" icon-size text-accent" />
									{/if}
									<div class={`flex-grow result-title ${selectedIndex === index ? 'text-accent' : ' text-text'}`}>
										{result.text?.text}
									</div>
									<div class={`result-alt ${selectedIndex === index ? 'text-accent' : ' text-sub-text'}`}>
										Alt + {index}
									</div>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>
