import { Settings } from "@pages/Settings/ViewModel";
import { invoke } from "@tauri-apps/api";
import { DialogField, WhiskersAction } from "@pages/Search/ViewModel";
import { SelectOption } from "@components/ComponentClasses";
import { DialogFilter, OpenDialogOptions, open } from "@tauri-apps/api/dialog";

export interface UiState {
  hasLoaded: boolean;
  settings: Settings | null;
  action: WhiskersAction | null;
  values: DialogResult[];
}

export class ViewModel {
  uiState: UiState = {
    hasLoaded: false,
    settings: null,
    action: null,
    values: [],
  };

  async load() {
    this.uiState.settings = await invoke("get_settings");
    this.uiState.action = await invoke("get_extension_dialog_action");

    let newValues: DialogResult[] = [];
    this.uiState.action?.fields.forEach((field) => {
      if (field.type === "Input") {
        this.uiState.values.push({
          id: field.id,
          value: field.value!!,
        });
      }
      if (field.type === "TextArea") {
        this.uiState.values.push({
          id: field.id,
          value: field.value!!,
        });
      }
      if (field.type === "Toggle") {
        this.uiState.values.push({
          id: field.id,
          value: field.toggled!!.toString(),
        });
      }
      if (field.type === "Select") {
        this.uiState.values.push({
          id: field.id,
          value: field.default_field_id!!,
        });
      }
      if (field.type === "SelectFile") {
        this.uiState.values.push({
          id: field.id,
          value: field.default_path ?? "",
        });
      }
    });
  }

  getSelectOptions(field: DialogField): SelectOption[] {
    let options: SelectOption[] = [];

    field.fields!!.forEach((selectField) => {
      options.push({
        id: selectField.id,
        text: selectField.text,
      });
    });

    return options;
  }

  getFieldValue(id: string): string {
    let value = "";

    this.uiState.values.forEach((v) => {
      if (v.id === id) {
        value = v.value;
      }
    });

    return value;
  }

  updateFieldValue(id: string, value: string) {
    
    let newValues: DialogResult[] = [];

    this.uiState.values.forEach((v) => {
      if (v.id === id) {
        newValues.push({
          id: v.id,
          value: value.toString(),
        });
      } else {
        newValues.push(v);
      }
    });

    this.uiState.values = newValues;
  }

  async selectFile(field: DialogField) {
    let options: OpenDialogOptions = {};

    if (field.select_dir) {
      options.directory = true;
    }

    if (field.default_path) {
      options.defaultPath = field.default_path;
    }

    if (field.filters) {
      let optionFilters: DialogFilter[] = [];

      field.filters.forEach((filter) => {
        optionFilters.push({
          name: filter.name,
          extensions: filter.file_extensions,
        });
      });

      options.filters = optionFilters;
    }

    const path = await open(options);

    if (!Array.isArray(path) && path !== null) {
      console.log(path);
      this.updateFieldValue(field.id, path);
    }
  }

  closeDialog() {
    invoke("close_extension_dialog", {
      extension_action: this.uiState.action?.extension_action,
      results: this.uiState.values,
    });
  }
}

export interface DialogResult {
  id: string;
  value: string;
}
