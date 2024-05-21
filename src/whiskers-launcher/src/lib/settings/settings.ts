import type { SelectValue } from '$lib/components/classes';
import { invoke } from '@tauri-apps/api';
import CssFilterConverter from 'css-filter-converter';

export interface Settings {
	first_key: string;
	second_key: string | null;
	third_key: string;
	scaling: number;
	auto_start: boolean;
	show_recent_apps: boolean;
	split_results: boolean;
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
	icon_path: string | null;
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

export interface App {
	id: string;
	title: string;
	icon: string | null;
	path: string;
}

export interface WLResult {
	result_type: string;
	text: TextResult | null;
	title_and_description: TitleAndDescriptionResult | null;
	divider: boolean;
}

export interface TextResult {
	icon: string | null;
	tint: string | null;
	text: string;
	action: Action;
}

export interface TitleAndDescriptionResult {
	icon: string | null;
	tint: string | null;
	title: string;
	description: string;
	action: Action;
}

export interface Action {
	action_type: string;
	open_app: OpenAppAction | null;
	open_url: OpenURLAction | null;
	copy: CopyAction | null;
	extension: ExtensionAction | null;
	dialog: DialogAction | null;
	ignore: boolean;
	ask_confirmation: boolean;
}

export interface OpenAppAction {
	id: string;
}

export interface OpenURLAction {
	url: string;
}

export interface CopyAction {
	text: string;
}

export interface ExtensionAction {
	extension_id: string;
	action: string;
	args: string[] | null;
}

export interface DialogAction {
	extension_id: String;
	action: String;
	title: String;
	action_text: String;
	fields: Field[];
	args: string[] | null;
}

export interface Field {
	field_type: string;
	input_field: InputField | null;
	text_area_field: TextAreaField | null;
	toggle_field: ToggleField | null;
	select_field: SelectField | null;
	file_picker_field: FilePickerField | null;
	args: string[] | null;
}

export interface InputField {
	id: string;
	default_value: string;
	title: string;
	description: string;
	placeholder: string;
}

export interface TextAreaField {
	id: string;
	default_value: string;
	title: string;
	description: string;
	placeholder: string;
}

export interface ToggleField {
	id: string;
	default_value: boolean;
	title: string;
	description: string;
}

export interface SelectField {
	id: string;
	default_value: string;
	title: string;
	description: string;
	options: SelectOption[];
}

export interface SelectOption {
	id: string;
	value: string;
}

export interface FilePickerField {
	id: string;
	title: string;
	description: string;
	default_path: string | null;
	filters: FileFilter[] | null;
	pick_directory: boolean;
}

export interface FileFilter {
	name: string;
	extensions: string[];
}

export interface Extension{
	
}

export async function getSettings(): Promise<Settings> {
	return await invoke('get_settings');
}

export function writeSettings(settings: Settings) {
	invoke('write_settings', { settings: settings });
}

export function getCssFilter(hex: string): string {
	let loss = 0;
	let attempts = 0;
	let filter = '';

	do {
		let result = CssFilterConverter.hexToFilter(hex);
		loss = result.loss ?? 0;
		filter = result.color ?? '';

		attempts += 1;

		if (attempts === 100) {
			break;
		}
	} while (loss > 0.1);

	return filter;
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
	--search-radius: ${settings.border_radius}px;
	--search-border-width: ${settings.border_width}px;
	--search-border-color: ${settings.accent_search_border ? settings.theme.accent : settings.theme.tertiary};
	--search-text-size: ${settings.scaling * 16}px;
	--result-icon-size: ${settings.scaling * 28}px;
	--search-icon-size: ${settings.scaling * 24}px;
	--result-title-size: ${settings.scaling * 16}px;
	--result-alt-size: ${settings.scaling * 14}px;
	--result-description-size: ${settings.scaling * 14}px;
	--result-divider-size: ${settings.scaling * 6}px;
}


.bg-background{
    background-color: var(--background);
}

.hover-background:hover{
	background-color: var(--background);
}

.bg-secondary{
    background-color: var(--secondary);
}

.hover-bg-secondary:hover{
	background-color: var(--secondary);
}

.bg-tertiary{
	background-color: var(--tertiary);
}

.bg-accent{
	background-color: var(--accent);
}

.bg-warning{
	background-color: var(--warning);
}

.bg-danger{
	background-color: var(--danger);
}

.bg-on-accent{
    background-color: var(--on-accent);
}

.bg-on-danger{
    background-color: var(--on-danger);
}

.bg-text{
    background-color: var(--text);
}

.bg-sub-text{
	background-color: var(--sub-text);
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

.accent-filter{
	filter: ${getCssFilter(settings.theme.accent)};
}

.text-on-accent{
	color: var(--on-accent);
}

.border-accent{
	border: 2px solid var(--accent);
}

.border-tertiary{
	border: 2px solid var(--tertiary);
}

.hover-border-accent:hover{
	border: 2px solid var(--accent);
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

.warning{
    border: 1px solid var(--warning);
}

.text-warning{
	color: var(--warning);
}

.v-divider{
	height: 1px;
	width: 100%;
	background-color: var(--tertiary);
}

.round{
	border-radius: ${settings.border_radius}px;
}

input::placeholder{
	color: var(--sub-text);
}

/*Search Bar*/
.search-text{
	font-size: var(--search-text-size);
}

/*Search Box*/

.search-round{
	border-radius: var(--search-radius);
}

.search-border{
    border: var(--search-border-width) solid var(--search-border-color);
}

.search-box-width{
	width: ${settings.scaling * 800}px;
}



.icon-size{
	height: var(--result-icon-size);
	width: var(--result-icon-size);
}

.search-icon-size{
    height: var(--search-icon-size);
    width: var(--search-icon-size);
}

.highlight-result{
	font-weight: 600;
	color: var(--accent);
	background-color: var(${settings.highlight_selected_background ? '--secondary' : '--background'});
}

.result-title{
    font-size: var(--result-title-size);
}

.result-alt{
	font-size: var(--result-alt-size);
}

.result-description{
    font-size: var(--result-description-size);
}

.result-divider{
	height: var(--result-divider-size);
	width: 100%;
}

</style>
    `;
}

export const LAUNCH_FIRST_KEY_OPTIONS: SelectValue[] = [
	{
		id: 'ctrl',
		value: 'Ctrl'
	},
	{
		id: 'alt',
		value: 'Alt'
	},
	{
		id: 'super',
		value: 'Super'
	},
	{
		id: 'shift',
		value: 'Shift'
	}
];

export const LAUNCH_SECOND_KEY_OPTIONS: SelectValue[] = [
	{
		id: '-',
		value: '-'
	},
	{
		id: 'alt',
		value: 'Alt'
	},
	{
		id: 'shift',
		value: 'Shift'
	},
	{
		id: 'super',
		value: 'Super'
	}
];

export const LAUNCH_THIRD_KEY_OPTIONS: SelectValue[] = [
	{
		id: 'space',
		value: 'Space'
	},
	{
		id: 'a',
		value: 'A'
	},
	{
		id: 'b',
		value: 'B'
	},
	{
		id: 'c',
		value: 'C'
	},
	{
		id: 'd',
		value: 'D'
	},
	{
		id: 'e',
		value: 'E'
	},
	{
		id: 'f',
		value: 'F'
	},
	{
		id: 'g',
		value: 'G'
	},
	{
		id: 'h',
		value: 'H'
	},
	{
		id: 'i',
		value: 'I'
	},
	{
		id: 'j',
		value: 'J'
	},
	{
		id: 'k',
		value: 'K'
	},
	{
		id: 'l',
		value: 'L'
	},
	{
		id: 'm',
		value: 'M'
	},
	{
		id: 'n',
		value: 'N'
	},
	{
		id: 'o',
		value: 'O'
	},
	{
		id: 'p',
		value: 'P'
	},
	{
		id: 'q',
		value: 'Q'
	},
	{
		id: 'r',
		value: 'R'
	},
	{
		id: 's',
		value: 'S'
	},
	{
		id: 't',
		value: 'T'
	},
	{
		id: 'u',
		value: 'U'
	},
	{
		id: 'v',
		value: 'V'
	},
	{
		id: 'w',
		value: 'W'
	},
	{
		id: 'x',
		value: 'X'
	},
	{
		id: 'y',
		value: 'Y'
	},
	{
		id: 'z',
		value: 'Z'
	}
];
