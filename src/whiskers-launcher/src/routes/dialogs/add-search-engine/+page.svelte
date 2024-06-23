<script lang="ts">
	import InputSetting from '$lib/components/input-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import {
		getSettings,
		getThemeCss,
		type SearchEngine,
		type Settings
	} from '$lib/settings/settings';
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';

	let settings: Settings | null = null;
	let css = '';

	let iconPath: string | null = null;
	let convertedIconPath: string | null = null;
	let tintIcon = false;
	let keyword = '';
	let name = '';
	let searchQuery = '';
	$: disableButton = keyword === '' || name === '' || searchQuery === '';

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
	});

	async function getIcon() {
		const path = await open({
			multiple: false,
			filters: [
				{
					name: 'Images',
					extensions: ['png', 'jpg', 'jpeg', 'svg']
				}
			]
		});

		if (path !== null) {
			iconPath = path.toString();
			convertedIconPath = `${convertFileSrc(path.toString())}?${Math.floor(Math.random() * 696969)}`;
		}
	}

	function clearIcon() {
		iconPath = null;
		convertedIconPath = null;
	}

	function updateTintIcon(event: CustomEvent<boolean>) {
		tintIcon = event.detail;
	}

	function updateKeyword(event: CustomEvent<string>) {
		keyword = event.detail;
	}

	function updateName(event: CustomEvent<string>) {
		name = event.detail;
	}

	function updateSearchQuery(event: CustomEvent<string>) {
		searchQuery = event.detail;
	}

	async function addSearchEngine() {
		let engine: SearchEngine = {
			id: await invoke('get_new_search_engine_id'),
			icon_path: iconPath,
			tint_icon: tintIcon,
			keyword: keyword,
			name: name,
			search_query: searchQuery
		};

        await invoke("add_search_engine", {engine: engine});

		await emit('refresh-search-engines');

		appWindow.close();
	}
</script>

{#if settings !== null}
	{@html css}
	<div class=" bg-background p-4 w-full h-screen overflow-auto text-text flex flex-col">
		<div class=" flex-grow overflow-auto space-y-4">
			<div class="card flex items-center gap-4">
				<div class=" flex-grow">
					<p class=" text-xl font-medium">Icon</p>
					<p class=" text-sub-text">The search engine icon</p>
				</div>
				<div class=" flex flex-col space-y-2">
					<button
						class=" bg-background h-[140px] w-[140px] hover-border-accent rounded-xl flex items-center justify-center"
						on:click={getIcon}
					>
						{#if convertedIconPath !== null}
							<img
								class={`w-[120px] h-[120px] ${tintIcon ? 'accent-filter' : ''}`}
								src={convertedIconPath}
								alt="Search Engine Icon"
							/>
						{/if}
					</button>
					{#if convertedIconPath !== null}
						<SecondaryButton text="Clear" on:click={clearIcon} />
					{/if}
				</div>
			</div>
			<ToggleSetting
				title="Tint Icon"
				description="When enabled, it tints the search engine icon"
				toggled={tintIcon}
				on:toggle={updateTintIcon}
			/>
			<InputSetting
				title="Keyword"
				description="The search engine keyword"
				placeholder="Type the extension keyword"
				value={keyword}
				on:input={updateKeyword}
			/>
			<InputSetting
				title="Name"
				description="The search engine name"
				placeholder="Type the extension name"
				value={name}
				on:input={updateName}
			/>

			<InputSetting
				title="Search Query"
				description="The search engine query. Use %s as placeholder for the search term."
				placeholder="Example: https://www.google.com/search?q=%s"
				value={searchQuery}
				on:input={updateSearchQuery}
			/>
		</div>
		<div class=" mt-4 flex justify-end">
			<PrimaryButton text="Add" disabled={disableButton} on:click={addSearchEngine} />
		</div>
	</div>
{/if}
