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

:root{
	--background: ${settings.theme.background};
	--secondary: ${settings.theme.secondary};
	--tertiary: ${settings.theme.tertiary};
	--accent: ${settings.theme.accent};
	--warning: ${settings.theme.warning};
	--danger: ${settings.theme.danger};
	--on-accent: ${settings.theme.on_accent};
	--on-danger: ${settings.theme.on_danger};
	--text: ${settings.theme.text};
	--sub-text: ${settings.theme.sub_text};
}

.bg-background{
    background-color: var(--background);
}

.bg-secondary{
    background-color: var(--secondary);
}

.hover-bg-secondary:hover{
	background-color: var(--secondary);
}

.hover-bg-tertiary:hover{
    background-color: var(--tertiary);
}

.text-text{
    color: var(--text);
}

.text-sub-text{
	color: var(--sub-text);
}

.text-accent{
    color: var(--accent);
}

.hover-text-accent:hover{
    color: var(--accent);
}

.card{
    background-color: var(--secondary);
    border: 1px solid var(--tertiary);
    padding: 12px;
	border-radius: 14px;
}

.input{
	background-color: var(--background);
	border: 1px solid var(--tertiary);
	padding: 8px;
	border-radius: 6px;
	width: 100%;
}

.input:focus{
	outline: 1px solid var(--accent);
}

.input::placeholder{
	color: var(--sub-text);
}

.select{
	background-color: var(--background);
	border: 1px solid var(--tertiary);
	border-radius: 6px;
	width: 100%;
}

.warning{
    border: 1px solid var(--warning);
}

.v-divider{
	height: 1px;
	width: 100%;
	background-color: var(--tertiary);
}

</style>
    `;
}
