<script lang="ts">
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import InputSetting from '$lib/components/input-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import TextAreaSetting from '$lib/components/text-area-setting.svelte';
	import SelectSetting from '$lib/components/select-setting.svelte';
	import FolderIcon from '$lib/icons/folder.svg?component';
	import {
		getSettings,
		getThemeCss,
		type DialogAction,
		type DialogResult,
		type Field,
		type FilePickerField,
		type SelectOption,
		type Settings
	} from '$lib/settings/settings';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import type { SelectValue } from '$lib/components/classes';
	import type { DialogFilter, OpenDialogOptions } from '@tauri-apps/api/dialog';
	import { open } from '@tauri-apps/api/dialog';

	let settings: Settings | null = null;
	let css = '';
	let action: DialogAction | null = null;
	let fields: Field[] = [];
	let results: DialogResult[] = [];

	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
		action = await invoke('get_dialog_request');
		fields = action!!.fields;

		console.log(JSON.stringify(fields));

		fields.forEach((field) => {
			let value = '';

			switch (field.field_type) {
				case 'Input':
					value = field.input_field!!.default_value;
					break;
				case 'TextArea':
					value = field.text_area_field!!.default_value;
					break;
				case 'Toggle':
					value = field.toggle_field!!.default_value.toString();
					break;
				case 'Select':
					value = field.select_field!!.default_value;
					break;
				case 'FilePicker':
					value = field.file_picker_field!!.default_path ?? '';
					break;
			}

			results.push({
				field_id: field.id,
				field_value: value,
				args: field.args
			});
		});
	});

	function getFieldValue(id: string): string {
		for (const index in results) {
			let result = results[index];

			if (result.field_id === id) {
				return result.field_value;
			}
		}

		return '';
	}

	function updateFieldValue(fieldId: string, value: string) {
		let newResults: DialogResult[] = [];

		results.forEach((result) => {
			if (result.field_id === fieldId) {
				let newResult = result;
				newResult.field_value = value;
				newResults.push(newResult);
			} else {
				newResults.push(result);
			}
		});

		results = [...newResults];
		fields = [...fields];
	}

	function getSelectValues(fieldId: string): SelectValue[] {
		let values: SelectValue[] = [];

		fields.forEach((field) => {
			if (field.id === fieldId) {
				field.select_field?.options.forEach((option) => {
					values.push({
						id: option.id,
						value: option.value
					});
				});
			}
		});

		return values;
	}

	function getBoolean(value: string): boolean {
		return value === 'true';
	}

	async function openPicker(id: string, field: FilePickerField) {
		let options: OpenDialogOptions = {};

		if (field.pick_directory) {
			options.directory = true;
		}

		if (field.default_path) {
			options.defaultPath = field.default_path;
		}

		if (field.filters) {
			let filters: DialogFilter[] = [];

			field.filters.forEach((filter) => {
				filters.push({
					name: filter.name,
					extensions: filter.extensions
				});
			});

			options.filters = filters;
		}

		const path = await open(options);

		if (path && !Array.isArray(path)) {
			updateFieldValue(id, path);
		}
	}

	function runAction() {
		invoke('run_dialog_action', { action: action!!, results: results });
	}
</script>

{#if settings !== null && action !== null}
	{@html css}
	<div class=" p-4 gap-4 bg-background text-text h-screen w-full flex flex-col">
		<div class=" flex-grow space-y-4 overflow-auto">
			{#each fields as field}
				{#if field.field_type === 'Input'}
					<InputSetting
						title={field.input_field?.title}
						description={field.input_field?.description}
						placeholder={field.input_field?.placeholder}
						value={getFieldValue(field.id)}
						on:input={(event) => {
							updateFieldValue(field.id, event.detail);
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
							updateFieldValue(field.id, event.detail);
						}}
					/>
				{/if}
				{#if field.field_type === 'Toggle'}
					<ToggleSetting
						title={field.toggle_field?.title ?? ''}
						description={field.toggle_field?.description ?? ''}
						toggled={getBoolean(getFieldValue(field.id))}
						on:toggle={(event) => {
							updateFieldValue(field.id, event.detail.toString());
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
							updateFieldValue(field.id, event.detail.id);
						}}
					/>
				{/if}
				{#if field.field_type === 'FilePicker'}
					<div class=" card space-y-2">
						<div>
							<p class=" text-xl font-medium">{field.file_picker_field?.title}</p>
							<p class=" text-sub-text">{field.file_picker_field?.description}</p>
						</div>
						<button
							class=" flex gap-4 w-full items-center bg-background p-2 rounded-md border-tertiary"
							on:click={() =>
								field.file_picker_field && openPicker(field.id, field.file_picker_field)}
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
		<div class=" flex justify-end">
			<PrimaryButton text={action.action_text} on:click={runAction} />
		</div>
	</div>
{/if}
