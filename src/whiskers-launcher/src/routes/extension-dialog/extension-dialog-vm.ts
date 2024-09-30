import type { SelectValue } from '$lib/components/classes';
import type { DialogAction, DialogResult, Field, FilePickerField } from '$lib/settings/settings';
import { invoke } from '@tauri-apps/api';
import type { DialogFilter, OpenDialogOptions } from '@tauri-apps/api/dialog';
import { get, writable } from 'svelte/store';
import { open } from '@tauri-apps/api/dialog';

export const state = writable({
	loading: true,
	action: {} as DialogAction,
	fields: [] as Field[],
	results: [] as DialogResult[]
});

export async function init() {
	let currentState = get(state);
	currentState.action = await invoke('get_dialog_request');
	currentState.fields = currentState.action.fields;

	currentState.fields.forEach((field) => {
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

		currentState.results.push({
			field_id: field.id,
			field_value: value,
			args: field.args
		});
	});

	currentState.loading = false;

	state.set(currentState);
}

export function getFieldValue(id: string): string {
	let results = get(state).results;

	for (const index in results) {
		let result = results[index];

		if (result.field_id === id) {
			return result.field_value;
		}
	}

	return '';
}

export function updateFieldValue(fieldId: string, value: string) {
	let newResults: DialogResult[] = [];
	let currentState = get(state);

	currentState.results.forEach((result) => {
		if (result.field_id === fieldId) {
			let newResult = result;
			newResult.field_value = value;
			newResults.push(newResult);
		} else {
			newResults.push(result);
		}
	});

	currentState.results = newResults;

	state.set(currentState);
}

export function getSelectValues(fieldId: string): SelectValue[] {
	let values: SelectValue[] = [];

	get(state).fields.forEach((field) => {
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

export function getBoolean(value: string): boolean {
	return value === 'true';
}

export async function openPicker(id: string, field: FilePickerField) {
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

export function onRunAction() {
	let currentState = get(state);
	invoke('run_dialog_action', { action: currentState.action, results: currentState.results });
}
