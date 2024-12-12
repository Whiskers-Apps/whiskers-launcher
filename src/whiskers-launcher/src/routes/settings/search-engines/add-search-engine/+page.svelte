<script lang="ts">
	import InputSetting from '$lib/components/input-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import {
		onClearIcon,
		onSelectIcon,
		onSetKeyword,
		onSetName,
		onSetSearchQuery,
		onSetTintIcon,
		onAddSearchEngine,
		state
	} from './add-search-engine-dialog-vm';
	import DialogFrame from '../../../dialog-frame.svelte';

	$: uiState = $state;
	$: disableButton = uiState.keyword === '' || uiState.name === '' || uiState.searchQuery === '';

</script>

<DialogFrame>
	<div class="w-full h-screen overflow-auto flex flex-col p-4">
		<div class=" flex-grow overflow-auto space-y-8">
			<div class="flex items-center gap-4">
				<div class=" flex-grow">
					<p class=" text-xl font-medium">Icon</p>
					<p class=" text-sub-text">The search engine icon</p>
				</div>
				<div class=" flex flex-col space-y-2">
					<button
						class=" bg-secondary h-[140px] w-[140px] hover-border-accent rounded-xl flex items-center justify-center"
						on:click={onSelectIcon}
					>
						{#if uiState.convertedIconPath !== null}
							<img
								class={`w-[120px] h-[120px] ${uiState.tintIcon ? 'accent-filter' : ''}`}
								src={uiState.convertedIconPath}
								alt="Search Engine Icon"
							/>
						{/if}
					</button>
					{#if uiState.convertedIconPath !== null}
						<SecondaryButton text="Clear" on:click={onClearIcon} />
					{/if}
				</div>
			</div>
			<ToggleSetting
				title="Tint Icon"
				description="When enabled, it tints the search engine icon"
				toggled={uiState.tintIcon}
				on:toggle={onSetTintIcon}
			/>
			<InputSetting
				title="Keyword"
				description="The search engine keyword"
				placeholder="Type the extension keyword"
				value={uiState.keyword}
				on:input={onSetKeyword}
			/>
			<InputSetting
				title="Name"
				description="The search engine name"
				placeholder="Type the extension name"
				value={uiState.name}
				on:input={onSetName}
			/>

			<InputSetting
				title="Search Query"
				description="The search engine query. Use %s as placeholder for the search term."
				placeholder="https://www.google.com/search?q=%s"
				value={uiState.searchQuery}
				on:input={onSetSearchQuery}
			/>
		</div>
		<div class=" mt-4 flex justify-end">
			<PrimaryButton text="Add" disabled={disableButton} on:click={onAddSearchEngine} />
		</div>
	</div>
</DialogFrame>
