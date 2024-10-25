import type { SelectValue } from '$lib/components/classes';
import type { FormField, OpenFormAction, RunExtensionAction } from '$lib/features/search/Search';
import { invoke } from '@tauri-apps/api';
import { get, writable } from 'svelte/store';
import isNumber from 'is-number';
import type { FormResponse, FormResult } from '$lib/features/form/Form';
import { appWindow } from '@tauri-apps/api/window';

export const state = writable({
	loading: true,
	fields: [] as FormField[],
	args: [] as string[],
	actionText: '',
	disableButton: true,
	extensionId: '',
	command: ''
});

export async function init() {
	let currentState = get(state);
	let action: OpenFormAction = await invoke('run_get_form_request');

	currentState.fields = action.fields;
	currentState.actionText = action.action_text;
	currentState.args = action.args;
	currentState.extensionId = action.extension_id;
	currentState.command = action.command;

	state.set(currentState);

	currentState.disableButton = !fieldsAreValid();
	currentState.loading = false;
	state.set(currentState);
}

export function setFieldValue(fieldId: string, value: string) {
	let currentState = get(state);
	let newFields: FormField[] = [];

	currentState.fields.forEach((field) => {
		let newField = field;

		if (field.id === fieldId) {
			switch (field.field_type) {
				case 'Input': {
					newField.input_field!!.text = value;
					break;
				}
				case 'TextArea': {
					newField.text_area_field!!.text = value;
					break;
				}
				case 'Toggle': {
					newField.toggle_field!!.toggled = value === 'true';
					break;
				}
				case 'Select': {
					newField.select_field!!.selected_option_id = value;
					break;
				}
				case 'FilePicker': {
					newField.file_picker_field!!.file_path = value.length === 0 ? null : value;
					break;
				}
				case 'FolderPicker': {
					newField.folder_picker_field!!.folder_path = value.length === 0 ? null : value;
					break;
				}
			}
		}

		newFields.push(newField);
	});

	currentState.fields = newFields;
	state.set(currentState);

	currentState.disableButton = !fieldsAreValid();
	state.set(currentState);

	console.log(fieldsAreValid());
}

export function getSelectValues(fieldId: string): SelectValue[] {
	let currentState = get(state);
	let values: SelectValue[] = [];

	currentState.fields.forEach((field) => {
		if (field.id === fieldId) {
			field.select_field?.options?.forEach((option) => {
				values.push({
					id: option.id,
					value: option.text
				});
			});
		}
	});

	return values;
}

function fieldsAreValid(): boolean {
	let currentState = get(state);

	for (const field of currentState.fields) {
		switch (field.field_type) {
			case 'Input': {
				if (field.input_field?.validation !== null) {
					for (const validation of field?.input_field?.validation!!) {
						if (validation === 'IsNotEmpty') {
							if (field.input_field?.text.trim().length === 0) {
								return false;
							}
						}

						if (validation === 'IsNumber') {
							if (!isNumber(field.input_field?.text)) {
								return false;
							}
						}
					}
				}
				break;
			}
			case 'TextArea': {
				if (field.text_area_field?.validation !== null) {
					if (field.text_area_field?.text.trim().length === 0) {
						return false;
					}
				}
				break;
			}
			case 'FilePicker': {
				if (field.file_picker_field?.validation !== null) {
					if (!field.file_picker_field?.file_path) {
						return false;
					}
				}
				break;
			}
			case 'FolderPicker': {
				if (field.folder_picker_field?.validation !== null) {
					if (!field.folder_picker_field?.folder_path) {
						return false;
					}
				}
				break;
			}
		}
	}

	return true;
}

export async function onRunAction() {
	let currentState = get(state);
	let results: FormResult[] = [];

	currentState.fields.forEach((field) => {
		let value = '';

		switch (field.field_type) {
			case 'Input': {
				value = field.input_field?.text ?? '';
				break;
			}
			case 'TextArea': {
				value = field.text_area_field?.text ?? '';
				break;
			}
			case 'Toggle': {
				value = field.toggle_field?.toggled === true ? 'true' : 'false';
				break;
			}
			case 'Select': {
				value = field.select_field?.selected_option_id ?? '';
				break;
			}
			case 'FilePicker': {
				value = field.file_picker_field?.file_path ?? '';
				break;
			}
			case 'FolderPicker': {
				value = field.folder_picker_field?.folder_path ?? '';
				break;
			}
		}

		results.push({
			field_id: field.id,
			field_value: value,
			args: field.args
		});
	});

	let formResponse: FormResponse = {
		results: results,
		args: currentState.args
	};

	await invoke('run_write_form_response', { response: formResponse });

	let action: RunExtensionAction = {
		extension_id: currentState.extensionId,
		command: currentState.command,
		args: []
	};

	await invoke('run_extension_action', { action: action });

	appWindow.close();
}
