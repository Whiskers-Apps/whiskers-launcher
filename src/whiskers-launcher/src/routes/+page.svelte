<script lang="ts">
	import SettingsIcon from '$lib/icons/settings.svg?component';
	import SearchIcon from '$lib/icons/search.svg?component';
	import {
		getCssFilter,
		getSettings,
		getThemeCss,
		type Settings,
		type WLResult
	} from '$lib/settings/settings';
	import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
	import { WebviewWindow, appWindow } from '@tauri-apps/api/window';
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
	let showConfirmationBox = false;

	// ===========================
	// UI Events
	// ===========================
	onMount(async () => {
		appWindow.setResizable(false);

		settings = await getSettings();
		css = getThemeCss(settings);

		search();
	});

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

		if (event.altKey && ['1', '2', '3', '4', '5', '6', '7', '8', '9'].includes(event.key)) {
			selectAltResult(+event.key - 1);
		}
	});

	async function handleBlur() {
		if (settings?.hide_on_blur) {
			appWindow.close();
		}
	}

	async function openSettings(event: Event | undefined = undefined) {
		event?.stopPropagation();

		invoke('open_settings_window');

		setTimeout(() => {
			appWindow.close();
		}, 200);
	}

	async function searchWithInput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		searchText = event.currentTarget.value;
		search();
	}

	async function search() {
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

	async function selectAltResult(index: number) {
		if (index > displayedResults.length) {
			return;
		}

		selectedIndex = index;
		runAction();
	}

	async function runAction() {
		let result = displayedResults[selectedIndex];

		invoke('run_action', { result: displayedResults[selectedIndex] });
	}

	function getColorFilter(tint: string | null): string {
		if (tint === null) {
			return 'none';
		}

		if (tint === 'accent') {
			return getCssFilter(settings!!.theme.accent);
		}

		return getCssFilter(tint);
	}
</script>

<div>
	{#if settings !== null}
		{@html css}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div class="h-screen overflow-hidden flex justify-center text-text pt-16" on:click={handleBlur}>
			<div class={`search-box-width h-fit search-round overflow-hidden ${settings.split_results ? '' : 'search-border bg-background'}`}>
				<div
					class={`flex p-3 gap-2 bg-background search-round ${settings.split_results ? 'search-border' : ''}`}
				>
					{#if settings.show_search_icon}
						<SearchIcon class=" search-icon-size text-accent" />
					{/if}

					<!-- svelte-ignore a11y-autofocus -->
					<input
						class=" w-full outline-none flex-grow bg-transparent search-text"
						type="text"
						placeholder={settings.show_placeholder ? 'Search apps, extensions, web' : ''}
						value={searchText}
						on:input={searchWithInput}
						autofocus
					/>

					{#if settings.show_settings_icon}
						<button
							class=" hover-bg-tertiary rounded-full"
							on:click={(event) => {
								openSettings(event);
							}}
						>
							<SettingsIcon class=" search-icon-size text-accent" />
						</button>
					{/if}
				</div>

				{#if settings.split_results}
					<div class="split-divider"></div>
				{/if}

				<div
					class={`search-round bg-background overflow-hidden ${settings.split_results ? 'search-border' : ''}`}
				>
					{#each displayedResults as result, index}
						<!-- svelte-ignore a11y-no-static-element-interactions -->
						<!-- svelte-ignore a11y-mouse-events-have-key-events -->
						<button
							class={`flex w-full items-center cursor-pointer ${index === selectedIndex ? 'highlight-result' : ''}`}
							on:mouseover={() => (selectedIndex = index)}
							on:focus={() => (selectedIndex = index)}
							on:click={runAction}
						>
							{#if result.result_type === 'Text'}
								<div class="w-full flex gap-4 p-3 items-center">
									{#if result.text?.icon !== null}
										<img
											class={`icon-size ${result.text?.tint}`}
											src={convertFileSrc(result.text?.icon ?? '')}
											alt=""
											style="filter: {getColorFilter(result.text?.tint ?? null)}"
										/>
									{/if}
									<div
										class={`flex-grow one-line text-start result-title ${selectedIndex === index ? 'text-accent' : ' text-text'}`}
									>
										{result.text?.text}
									</div>
									{#if settings.show_alt_hint}
										<div
											class={`result-alt ${selectedIndex === index ? 'text-accent' : ' text-sub-text'}`}
										>
											Alt + {index + 1}
										</div>
									{/if}
								</div>
							{/if}
							{#if result.result_type === 'TitleAndDescription'}
								<div class="w-full flex gap-4 p-3 items-center">
									{#if result.title_and_description?.icon !== null}
										<img
											class={`icon-size ${result.title_and_description?.tint}`}
											src={convertFileSrc(result.title_and_description?.icon ?? '')}
											alt=""
											style="filter: {getColorFilter(result.title_and_description?.tint ?? null)}"
										/>
									{/if}
									<div
										class={`flex-grow one-line text-start result-title flex flex-col ${selectedIndex === index ? 'text-accent' : ' text-text'}`}
									>
										<p class="result-title one-line">{result.title_and_description?.title}</p>
										<p class="result-description one-line">
											{result.title_and_description?.description}
										</p>
									</div>
									{#if settings.show_alt_hint}
										<div
											class={`result-alt ${selectedIndex === index ? 'text-accent' : ' text-sub-text'}`}
										>
											Alt + {index + 1}
										</div>
									{/if}
								</div>
							{/if}
							{#if result.result_type === 'Divider'}
								<div class="p-2 pt-4 pb-4 w-full">
									<div
										class={`result-divider rounded-full ${selectedIndex === index ? 'bg-accent' : 'bg-tertiary'}`}
									></div>
								</div>
							{/if}
						</button>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>
