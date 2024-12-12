import type { SelectValue } from "$lib/components/classes";
import { invoke } from "@tauri-apps/api";

export interface Settings {
	first_key: string;
	second_key: string | null;
	third_key: string;
	auto_start: boolean;
	show_recent_apps: boolean;
	show_search_icon: boolean;
	show_settings_icon: boolean;
	show_placeholder: boolean;
	hide_on_blur: boolean;
	border_radius: number;
	border_width: number;
	accent_border: boolean;
	show_launch_hint: boolean;
	launch_key: string;
	blacklist: string[];
	search_keyword: string;
	search_engines: SearchEngine[];
	default_search_engine: number;
	theme: Theme;
	extensions: ExtensionSetting[];
	wallpaper: string | null;
	show_apps_as_grid: boolean;
	hide_app_icons: boolean;
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

export interface ExtensionStoreItem {
	id: string;
	name: string;
	description: string;
	repo: string;
	preview: string;
	os: string[] | null;
}

export interface ThemeStoreItem {
	id: string;
	name: string;
	repo: string;
	preview: string;
	file: string | null;
	variants: ThemeStoreVariant[] | null;
}

export interface ThemeStoreVariant{
	name: string;
	file: string;
}

export async function getSettings(): Promise<Settings> {
	return await invoke('run_get_settings');
}

export async function writeSettings(settings: Settings) {
	invoke('run_write_settings', { settings: settings });
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
