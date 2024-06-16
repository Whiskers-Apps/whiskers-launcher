<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import Select from '$lib/components/select.svelte';
	import {
		getSettings,
		getThemeCss,
		writeSettings,
		type Extension,
		type Settings,
		type ThemeStoreItem,
		type ThemeStoreVariant
	} from '$lib/settings/settings';
	import ChevronRightIcon from '$lib/icons/chevron-right.svg?component';
	import ChevronLeftIcon from '$lib/icons/chevron-left.svg?component';
	import { invoke } from '@tauri-apps/api';
	import { open } from '@tauri-apps/api/shell';
	import axios from 'axios';
	import { onMount } from 'svelte';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import type { SelectValue } from '$lib/components/classes';
	import { emit } from '@tauri-apps/api/event';

	let settings: Settings | null = null;
	let css = '';
	let searchText = '';
	let store: ThemeStoreItem[] = [];
	let filteredStore: ThemeStoreItem[] = [];
	let displayedStore: ThemeStoreItem[] = [];
	let page = 0;
	let applyingTheme = false;
	let listingSelectValues: ListingSelectValue[] = [];

	interface ListingSelectValue {
		theme_id: string;
		value_id: string;
	}

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);

		// Cached Store
		store = await invoke('get_themes_store');
		filteredStore = store;
		displayedStore = store.slice(0, 12);
		indexSelectValues();

		await axios
			.get(
				'https://raw.githubusercontent.com/Whiskers-Apps/whiskers-launcher-themes/master/themes.json'
			)
			.then((response) => {
				store = [...response.data];
				filteredStore = store;

				displayedStore = [...store.slice(0, 12)];

				indexSelectValues();

				invoke('write_themes_store', { store: store });
			})
			.catch((error) => console.error(error));
	});

	async function indexSelectValues() {
		let newValues: ListingSelectValue[] = [];
		store.forEach((listing) => {
			if (listing.variants) {
				newValues.push({ theme_id: listing.id, value_id: listing.variants!![0].file });
			}
		});
		listingSelectValues = [...newValues];
	}

	async function search() {
		if (searchText.length === 0) {
			page = 0;
			filteredStore = [...store];
			displayedStore = [...filteredStore.slice(0, 12)];
			return;
		}

		let searchTerm = searchText.toLowerCase();

		filteredStore = store
			.filter((item) => item.name.toLowerCase().includes(searchTerm))
			.slice(0, 12);

		page = 0;

		displayedStore = [...filteredStore];
	}

	function getSelectValues(variants: ThemeStoreVariant[]): SelectValue[] {
		return variants.map((variant) => ({
			id: variant.file,
			value: variant.name
		}));
	}

	function getSelectValueId(id: string): string {
		for (let value of listingSelectValues) {
			if (value.theme_id === id) {
				return value.value_id;
			}
		}

		return '';
	}

	function setSelectValue(theme_id: string, value_id: string) {
		let newListing = listingSelectValues.map((value) =>
			value.theme_id === theme_id ? { theme_id: theme_id, value_id: value_id } : value
		);
		listingSelectValues = [...newListing];
		displayedStore = [...displayedStore];
	}

	async function applyTheme(theme: ThemeStoreItem) {
		try {
			let repo = theme.file ? theme.file : getSelectValueId(theme.id);

			axios
				.get(repo)
				.then(async (response) => {
					let json = response.data;
					let settings = await getSettings();
					settings.theme = {
						background: json.background,
						secondary: json.secondary,
						tertiary: json.tertiary,
						accent: json.accent,
						warning: json.warning,
						danger: json.danger,
						on_accent: json.on_accent,
						on_danger: json.on_danger,
						text: json.text,
						sub_text: json.sub_text
					};

					console.log(settings)

					writeSettings(settings);

					css = getThemeCss(settings);

					emit('refresh-theme');
				})
				.catch((error) => console.error(error));
		} catch (error) {
			console.error(error);
		}
	}

	async function goToPreviousPage() {
		page -= 1;
		displayedStore = [...filteredStore.slice(page * 12, (page + 1) * 12)];
	}

	async function goToNextPage() {
		page += 1;
		displayedStore = [...filteredStore.slice(page * 12, (page + 1) * 12)];
	}
</script>

{#if settings !== null}
	<div>
		{@html css}
		<div class=" bg-background p-4 h-screen w-full text-text space-y-4 flex flex-col">
			<Input
				value={searchText}
				placeholder="Search themes"
				on:input={(e) => {
					searchText = e.detail;
					search();
				}}
			/>

			<div class=" flex-grow overflow-auto space-y-2">
				{#each displayedStore as storeListing}
					<div class="p-4 border border-tertiary bg-secondary rounded-lg space-y-2 w-full">
						<div class="flex justify-center bg-background rounded-lg">
							<img
								class="h-52 object-contain"
								src={storeListing.preview}
								alt={`Extension, ${storeListing.preview}`}
							/>
						</div>
						<div class=" text-xl font-medium text-start">{storeListing.name}</div>
						<div class="flex items-start">
							<div class="text-start flex flex-col gap-2 flex-grow">
								{#if storeListing.variants !== null}
									<Select
										values={getSelectValues(storeListing.variants)}
										selectedValue={getSelectValueId(storeListing.id)}
										on:selection={(e) => setSelectValue(storeListing.id, e.detail.id)}
									/>
								{/if}
								<div class="flex justify-end space-x-2">
									<SecondaryButton text="Source Code" on:click={() => open(storeListing.repo)} />
									<SecondaryButton
										text="Apply"
										disabled={applyingTheme}
										on:click={() => applyTheme(storeListing)}
									/>
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>

			<div class="flex justify-end space-x-4 items-center">
				<button
					class={`p-2  rounded-md ${page == 0 ? 'bg-tertiary text-sub-text' : 'bg-accent text-on-accent'}`}
					disabled={page === 0}
					on:click={goToPreviousPage}
				>
					<ChevronLeftIcon class={` h-6 w-6 ${page === 0}`} />
				</button>
				<div class="p-2 pr-4 pl-4 bg-tertiary rounded-md">{page + 1}</div>
				<button
					class={`p-2  rounded-md ${(page + 1) * 12 + 1 <= filteredStore.length ? 'bg-accent text-on-accent' : 'bg-tertiary text-sub-text'}`}
					disabled={(page + 1) * 12 > filteredStore.length}
					on:click={goToNextPage}
				>
					<ChevronRightIcon class=" h-6 w-6" />
				</button>
			</div>
		</div>
	</div>
{/if}
