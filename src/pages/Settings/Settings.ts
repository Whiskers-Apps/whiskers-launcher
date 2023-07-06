import { invoke } from "@tauri-apps/api"

export const SettingsCategory = {
  GENERAL: "general",
  SEARCH_BOX: "search_box",
  THEMING: "theming",
  WEB_SEARCH: "web_search",
  EXTENSIONS: "extensions"
};

export interface Settings {
  general: GeneralSettings,
  search_box: SearchBoxSettings,
  theming: ThemingSettings,
  web_search: WebSearchSettings,
  extensions: ExtensionSettings[]
}

export interface ExtensionSettings{
  id: string,
  keyword: string,
  settings: ExtensionSetting
}

export interface ExtensionSetting{
  any: ExtensionOptionSetting[],
  linux: ExtensionOptionSetting[],
  windows: ExtensionOptionSetting[]
}

export interface ExtensionOptionSetting{
  id: string,
  current_value: string
}

export interface GeneralSettings {
  first_key: string;
  second_key: string;
  third_key: string;
  limit: number;
}

export interface SearchBoxSettings {
  show_search_icon: boolean;
  show_settings_icon: boolean;
  roundness: number;
  border_width: number;
}

export interface ThemingSettings {
  background: string;
  secondary_background: string;
  tertiary_background: string;
  accent: string;
  on_accent: string;
  text: string;
  seconday_text: string;
}

export interface WebSearchSettings{
  default: SearchOption[],
  custom: SearchOption[]
}

export interface SearchOption{
  icon: string,
  name: string,
  keyword: string,
  query: string
}



export async function getSettings(): Promise<Settings> {

  let settings: Settings = JSON.parse(await invoke("get_current_settings"));
  return settings
}

export async function updateSetting(setting: string, newValue: any) {

  var settings = await getSettings();

  switch (setting) {
    case "general_first_key": { settings.general.first_key = newValue; break }
    case "general_second_key": { settings.general.second_key = newValue; break }
    case "general_third_key": { settings.general.third_key = newValue; break }
    case "general_limit": { settings.general.limit = newValue; break }
    case "search_box_show_search_icon": { settings.search_box.show_search_icon = newValue; break }
    case "search_box_show_settings_icon": { settings.search_box.show_settings_icon = newValue; break }
    case "search_box_roundness": { settings.search_box.roundness = newValue; break }
    case "search_box_border_width": { settings.search_box.border_width = newValue; break }
    case "theming_background": { settings.theming.background = newValue; break }
    case "theming_secondary_background": { settings.theming.secondary_background = newValue; break }
    case "theming_tertiary_background": { settings.theming.tertiary_background = newValue; break }
    case "theming_accent": { settings.theming.accent = newValue; break }
    case "theming_on_accent": { settings.theming.on_accent = newValue; break }
    case "theming_text": { settings.theming.text = newValue; break }
    case "theming_secondary_text": { settings.theming.seconday_text = newValue; break }

    default: { break }
  }

  invoke("update_settings", { settings_json: JSON.stringify(settings) });
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
  else if (roundness == 9) { return "36px" }
  else { return "9999px" }
}