import { invoke } from "@tauri-apps/api"
import { emit } from "@tauri-apps/api/event"


export enum SettingsTab{
  General,
  Search,
  Results,
  Theme,
  SearchEngines,
  Extensions,
  About
}

export interface Settings {
  general: GeneralSettings,
  search: SearchSettings,
  results: ResultsSettings,
  theme: ThemeSettings,
  search_engines: SearchEngineSettings[],
  extensions: ExtensionSettings[]
}



export interface GeneralSettings {
  first_key: string;
  second_key: string;
  third_key: string;
  auto_start: boolean;
}

export interface SearchSettings {
  show_settings_icon: boolean,
  show_search_icon: boolean,
  show_placeholder: boolean,
  border_radius: number,
  border_width: number
}

export interface ResultsSettings{
  results_count: number,
  split_ui: boolean,
  layout: TypeEnum
}

export interface TypeEnum{
  type: string
}

export interface ThemeSettings {
  background: string;
  secondary_background: string;
  tertiary_background: string;
  accent: string;
  on_accent: string;
  danger: string;
  on_danger: string;
  text: string;
  secondary_text: string;
}

export interface SearchEngineSettings {
  keyword: string,
  icon?: string,
  tint_icon: boolean,
  name: string,
  query: string,
  default: boolean
}

export interface ExtensionSettings {
  id: string,
  keyword: string,
  settings: ExtensionSetting
}

export interface ExtensionSetting {
  any: ExtensionOptionSetting[],
  linux: ExtensionOptionSetting[],
  windows: ExtensionOptionSetting[]
}

export interface ExtensionOptionSetting {
  id: string,
  current_value: string
}


export async function getSettings(): Promise<Settings> {

  let settings: Settings = JSON.parse(await invoke("get_current_settings"));

  return settings
}

export async function getTheme(): Promise<ThemeSettings> {
  let settings = await getSettings();

  return settings.theme;
}

export async function updateSettings(settings: Settings) {

  let settingsJson = JSON.stringify(settings);
  invoke("update_settings", { settings_json: settingsJson })
  emit("updateSettings");
}

export function getRoundnessInPixels(roundness: number): string {
  if (roundness == 0) { return "0px" }
  else if (roundness == 1) { return "4px" }
  else if (roundness == 2) { return "8px" }
  else if (roundness == 3) { return "12px" }
  else if (roundness == 4) { return "16px" }
  else if (roundness == 5) { return "20px" }
  else if (roundness == 6) { return "24px" }
  else if (roundness == 7) { return "28px" }
  else if (roundness == 8) { return "32px" }
  else { return "36px" }
}