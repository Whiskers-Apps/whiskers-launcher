import { invoke } from '@tauri-apps/api';

export interface Settings {
	first_key: string;
	second_key: string | null;
	third_key: string;
	auto_start: boolean;
	show_recent_apps: boolean;
	show_search_icon: boolean;
	show_settings_icon: boolean;
	show_placeholder: boolean;
	accent_search_border: boolean;
	hide_on_blur: boolean;
	border_radius: number;
	border_width: number;
	highlight_selected_background: boolean;
	show_alt_hint: boolean;
	results_count: number;
	blacklist: string[];
	search_keyword: string;
	search_engines: SearchEngine[];
	default_search_engine: number;
	theme: Theme;
	extensions: ExtensionSetting[];
}

export interface SearchEngine {
	id: number;
	icon_path: string;
	tint_icon: boolean;
	keyword: string;
	name: string;
	search_query: string;
}

export interface Theme {
	background: string;
	secondary: string;
	tertiary: string;
	accent: string;
	warning: string;
	danger: string;
	on_accent: string;
	on_danger: string;
	text: string;
	sub_text: string;
}

export interface ExtensionSetting {
	extension_id: string;
	setting_id: string;
	setting_value: string;
}

export async function getSettings(): Promise<Settings> {
	return await invoke('get_settings');
}

export function getThemeCss(settings: Settings): string {
	return `
<style>

.bg-background{
    background-color: ${settings.theme.background};
}

.bg-secondary{
    background-color: ${settings.theme.secondary};
}

.hover-bg-tertiary:hover{
    background-color: ${settings.theme.tertiary};
}

.text-text{
    color: ${settings.theme.text};
}

.text-sub-text{
	color: ${settings.theme.sub_text};
}

.text-accent{
    color: ${settings.theme.accent};
}

.hover-text-accent:hover{
    color: ${settings.theme.accent};
}

.card{
    background-color: ${settings.theme.secondary};
    border: 1px solid ${settings.theme.tertiary};
    padding: 12px;
	border-radius: 14px;
}

.warning{
    border: 1px solid ${settings.theme.warning};
}

</style>
    `;
}
