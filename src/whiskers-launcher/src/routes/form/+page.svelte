<script lang="ts">
	import { onMount } from 'svelte';
	import DialogFrame from '../dialog-frame.svelte';
	import { getSelectValues, init, onRunAction, setFieldValue, state } from './form-vm';
	import InputSetting from '$lib/components/input-setting.svelte';
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import TextAreaSetting from '$lib/components/text-area-setting.svelte';
	import SelectSetting from '$lib/components/select-setting.svelte';
	import FilePickerSetting from '$lib/components/file-picker-setting.svelte';
	import FolderPickerSetting from '$lib/components/folder-picker-setting.svelte';

	$: uiState = $state;

	onMount(async () => {
		init();
	});
</script>

<DialogFrame>
	{#if !uiState.loading}
		<div class="h-screen w-full p-4 flex flex-col">
			<div class="flex-grow space-y-4 overflow-auto border-tertiary p-4 rounded-2xl">
				{#each uiState.fields as field}
					{#if field.field_type === 'Input'}
						<InputSetting
							title={field.input_field?.title ?? ''}
							description={field.input_field?.description ?? ''}
							value={field.input_field?.text ?? ''}
							on:input={(e) => setFieldValue(field.id, e.detail)}
						/>
					{/if}
					{#if field.field_type === 'TextArea'}
						<TextAreaSetting
							title={field.text_area_field?.title ?? ''}
							description={field.text_area_field?.description ?? ''}
							value={field.text_area_field?.text ?? ''}
							on:input={(e) => setFieldValue(field.id, e.detail)}
						/>
					{/if}
					{#if field.field_type === 'Toggle'}
						<ToggleSetting
							title={field.toggle_field?.title ?? ''}
							description={field.toggle_field?.description ?? ''}
							toggled={field.toggle_field?.toggled ?? false}
							on:toggle={(e) => setFieldValue(field.id, e.detail.toString())}
						/>
					{/if}
					{#if field.field_type === 'Select'}
						<SelectSetting
							title={field.select_field?.title ?? ''}
							description={field.select_field?.description ?? ''}
							values={getSelectValues(field.id)}
							selectedValue={field.select_field?.selected_option_id ?? ''}
							on:selection={(e) => setFieldValue(field.id, e.detail.id)}
						/>
					{/if}
					{#if field.field_type === 'FilePicker'}
						<FilePickerSetting
							title={field.file_picker_field?.title ?? ''}
							description={field.file_picker_field?.description ?? ''}
							selectedPath={field.file_picker_field?.file_path}
							fileTypes={field.file_picker_field?.file_types}
							on:file-changed={(e) => setFieldValue(field.id, e.detail ?? "")}
						/>
					{/if}
					{#if field.field_type === 'FolderPicker'}
						<FolderPickerSetting
							title={field.folder_picker_field?.title ?? ''}
							description={field.folder_picker_field?.description ?? ''}
							selectedPath={field.folder_picker_field?.folder_path}
							on:file-changed={(e) => setFieldValue(field.id, e.detail ?? "")}
						/>
					{/if}
				{/each}
			</div>
			<div class="flex justify-end mt-4">
				<PrimaryButton text={uiState.actionText} disabled={uiState.disableButton} on:click={onRunAction} />
			</div>
		</div>
	{/if}
</DialogFrame>
