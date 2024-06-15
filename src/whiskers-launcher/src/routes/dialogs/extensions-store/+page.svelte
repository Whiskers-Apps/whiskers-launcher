<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import {
		getSettings,
		getThemeCss,
		type Extension,
		type ExtensionStoreItem,
		type Settings
	} from '$lib/settings/settings';
	import ChevronRightIcon from '$lib/icons/chevron-right.svg?component';
	import ChevronLeftIcon from '$lib/icons/chevron-left.svg?component';
	import { invoke } from '@tauri-apps/api';
	import { open } from '@tauri-apps/api/shell';
	import axios from 'axios';
	import { onMount } from 'svelte';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { emit } from '@tauri-apps/api/event';

	let settings: Settings | null = null;
	let css = '';
	let searchText = '';
	let store: ExtensionStoreItem[] = [];
	let filteredStore: ExtensionStoreItem[] = [];
	let displayedStore: ExtensionStoreItem[] = [];
	let page = 0;
	let installedExtensions: String[] = [];
	let installingExtension = false;

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
		let extensions: Extension[] = await invoke('get_extensions');
		extensions.forEach((extension) => {
			installedExtensions.push(extension.id);
		});

		// Cached Store
		store = await invoke('get_extensions_store');
		store = store.concat(store);
		store = store.concat(store);
		filteredStore = store;
		displayedStore = store.slice(0, 12);

		axios
			.get(
				'https://raw.githubusercontent.com/Whiskers-Apps/whiskers-launcher-extensions/main/extensions.json'
			)
			.then((response) => {
				store = [...response.data];
				store = store.concat(store);
				store = store.concat(store);
				filteredStore = store;
				displayedStore = [...store.slice(0, 12)];

				invoke('write_extensions_store', { store: store });
			})
			.catch((error) => console.error(error));
	});

	async function search() {
		if (searchText.length === 0) {
			page = 0;
			filteredStore = [...store];
			displayedStore = [...filteredStore.slice(0, 12)];
			return;
		}

		let searchTerm = searchText.toLowerCase();

		filteredStore = store
			.filter(
				(item) =>
					item.name.toLowerCase().includes(searchTerm) ||
					item.description.toLowerCase().includes(searchTerm)
			)
			.slice(0, 12);

		page = 0;

		displayedStore = [...filteredStore];
	}

	async function installExtension(repo: string) {
		try {
			installingExtension = true;
			await invoke('clone_extension', { url: repo });
			installingExtension = false;

			installedExtensions = [];
			let extensions: Extension[] = await invoke('get_extensions');
			extensions.forEach((extension) => {
				installedExtensions.push(extension.id);
			});

			store = await invoke('get_extensions_store');
			search();

			emit('refresh-extensions');
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
				placeholder="Search extensions"
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
								<div class="flex-grow">
									{storeListing.description}
								</div>
								<div class="flex justify-end space-x-2">
									<SecondaryButton text="Source Code" on:click={() => open(storeListing.repo)} />
									{#if !installedExtensions.includes(storeListing.id)}
										<SecondaryButton
											text="Install"
											disabled={installingExtension}
											on:click={() => installExtension(storeListing.repo)}
										/>
									{/if}
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
