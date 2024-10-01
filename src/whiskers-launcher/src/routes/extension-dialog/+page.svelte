<script lang="ts">
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import InputSetting from '$lib/components/input-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import TextAreaSetting from '$lib/components/text-area-setting.svelte';
	import SelectSetting from '$lib/components/select-setting.svelte';
	import FolderIcon from '$lib/icons/folder.svg?component';
	import { onMount } from 'svelte';
	import DialogFrame from '../dialog-frame.svelte';
	import {
		getFieldValue,
		init,
		state,
		updateFieldValue as onSetFieldValue,
		getBoolean,
		getSelectValues,
		openPicker as onOpenFilePicker,
		onRunAction
	} from './extension-dialog-vm';

	$: uiState = $state;

	onMount(async () => {
		init();
	});
</script>

<DialogFrame>
	{#if !uiState.loading}
		<div class="  gap-4 bg-background text-text h-screen w-full flex flex-col">
			<div class=" flex-grow space-y-8 overflow-auto p-4">
				{#each uiState.fields as field}
					{#if field.field_type === 'Input'}
						<InputSetting
							title={field.input_field?.title}
							description={field.input_field?.description}
							placeholder={field.input_field?.placeholder}
							value={getFieldValue(field.id)}
							on:input={(event) => {
								onSetFieldValue(field.id, event.detail);
							}}
						/>
					{/if}
					{#if field.field_type === 'TextArea'}
						<TextAreaSetting
							title={field.text_area_field?.title}
							description={field.text_area_field?.description}
							placeholder={field.text_area_field?.placeholder}
							value={getFieldValue(field.id)}
							on:input={(event) => {
								onSetFieldValue(field.id, event.detail);
							}}
						/>
					{/if}
					{#if field.field_type === 'Toggle'}
						<ToggleSetting
							title={field.toggle_field?.title ?? ''}
							description={field.toggle_field?.description ?? ''}
							toggled={getBoolean(getFieldValue(field.id))}
							on:toggle={(event) => {
								onSetFieldValue(field.id, event.detail.toString());
							}}
						/>
					{/if}
					{#if field.field_type === 'Select'}
						<SelectSetting
							title={field.select_field?.title ?? ''}
							description={field.select_field?.description ?? ''}
							values={getSelectValues(field.id)}
							selectedValue={getFieldValue(field.id)}
							on:selection={(event) => {
								onSetFieldValue(field.id, event.detail.id);
							}}
						/>
					{/if}
					{#if field.field_type === 'FilePicker'}
						<div class="space-y-2">
							<div>
								<p class=" text-xl font-medium">{field.file_picker_field?.title}</p>
								<p class=" text-sub-text">{field.file_picker_field?.description}</p>
							</div>
							<button
								class=" flex gap-4 w-full items-center bg-background p-2 rounded-md border-tertiary card"
								on:click={() =>
									field.file_picker_field && onOpenFilePicker(field.id, field.file_picker_field)}
							>
								<FolderIcon class=" h-8 w-8" />
								<p class=" one-line">
									{getFieldValue(field.id) === ''
										? `Click here to select a ${field.file_picker_field?.pick_directory ? 'directory.' : 'file.'}`
										: getFieldValue(field.id)}
								</p>
							</button>
						</div>
					{/if}
				{/each}
			</div>
			<div class=" flex justify-end p-4">
				<PrimaryButton text={uiState.action.action_text} on:click={onRunAction} />
			</div>
		</div>
	{/if}
</DialogFrame>
