import type { SelectValue } from '$lib/components/classes';
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

.border-accent{
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
