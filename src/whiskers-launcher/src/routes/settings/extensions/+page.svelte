<script lang="ts">
	import { onMount } from 'svelte';
	import MainFrame from '../main-frame.svelte';
	import { canShowSetting, getExtensionSettingValue, init, onDeleteExtension, onOpenCloneExtensionDialog, onOpenExtensionDir, onOpenStore, onReloadExtensions, onUpdateSetting, state } from './extension-page-vm';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import FolderIcon from '$lib/icons/folder.svg?component';
	import TrashIcon from '$lib/icons/trash.svg?component';
	import Input from '$lib/components/input.svelte';
	import TextArea from '$lib/components/text-area.svelte';
	import Toggle from '$lib/components/toggle.svelte';
	import Select from '$lib/components/select.svelte';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class="space-y-8">
			<div class="flex">
				<SecondaryButton text="Extensions Store" on:click={onOpenStore} />
				<SecondaryButton text="Git Clone" on:click={onOpenCloneExtensionDialog} />
				<SecondaryButton text="Reload" on:click={onReloadExtensions} />
			</div>
			<div class=" space-y-8">
				{#each uiState.extensions as extension}
					<div>
						<div class=" flex">
							<p class=" flex-grow text-2xl font-bold">{extension.name}</p>
							<button
								class=" p-2 hover-bg-tertiary text-accent rounded-full hover-text-on-accent hover-bg-accent"
								on:click={() => onOpenExtensionDir(extension.id)}
							>
								<FolderIcon class=" " width="24" height="24" />
							</button>
							<button
								class=" p-2 hover-bg-tertiary text-danger rounded-full hover-text-on-danger hover-bg-danger"
								on:click={() => onDeleteExtension(extension.id)}
							>
								<TrashIcon class=" " width="24" height="24" />
							</button>
						</div>
						<p class=" text-sub-text">{extension.description}</p>
						<div class=" space-y-4">
							<div>
								<div class=" font-medium">Keyword</div>
								<div class=" text-sub-text">The extension keyword</div>
								<Input
									value={getExtensionSettingValue(extension.id, 'keyword')}
									on:input={(event) => {
										onUpdateSetting(extension.id, 'keyword', event.detail);
									}}
								/>
							</div>
							{#if extension.settings !== null}
								{#each extension.settings as setting}
									{#if canShowSetting(extension.id, setting)}
										<div>
											{#if setting.setting_type === 'Input'}
												<div class=" font-medium">{setting.title}</div>
												<div class=" text-sub-text">{setting.description}</div>

												<Input
													value={getExtensionSettingValue(extension.id, setting.id)}
													on:input={(event) => {
														onUpdateSetting(extension.id, setting.id, event.detail);
													}}
												/>
											{/if}
											{#if setting.setting_type === 'TextArea'}
												<div class=" font-medium">{setting.title}</div>
												<div class=" text-sub-text">{setting.description}</div>

												<TextArea
													value={getExtensionSettingValue(extension.id, setting.id)}
													on:input={(event) => {
														onUpdateSetting(extension.id, setting.id, event.detail);
													}}
												/>
											{/if}
											{#if setting.setting_type === 'Select'}
												<div class=" font-medium">{setting.title}</div>
												<div class=" text-sub-text">{setting.description}</div>

												<Select
													values={setting.select_options ?? []}
													selectedValue={getExtensionSettingValue(extension.id, setting.id)}
													on:selection={(event) => {
														onUpdateSetting(extension.id, setting.id, event.detail.id);
													}}
												/>
											{/if}
											{#if setting.setting_type === 'Toggle'}
												<div class="flex">
													<div class="flex-grow">
														<div class=" font-medium">{setting.title}</div>
														<div class=" text-sub-text">{setting.description}</div>
													</div>
													<Toggle
														toggled={getExtensionSettingValue(extension.id, setting.id) === 'true'
															? true
															: false}
														on:toggle={(e) => {
															onUpdateSetting(extension.id, setting.id, String(e.detail));
														}}
													/>
												</div>
											{/if}
										</div>
									{/if}
								{/each}
							{/if}
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</MainFrame>
