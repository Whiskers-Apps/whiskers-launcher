import { convertFileSrc } from '@tauri-apps/api/tauri';
import type { Settings } from '../settings/Settings';
import CssFilterConverter from 'css-filter-converter';

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
	--search-border-color: ${settings.accent_border ? settings.theme.accent : settings.theme.tertiary};
}

/* ================== Images ================== */

.wallpaper{
	background-image: url(${settings.wallpaper ? convertFileSrc(settings.wallpaper) : 'none'});
	width: 100vw;
	height: 100vh;
	background-size: cover;
	background-position: center;
}

.accent-filter{
	filter: ${getCssFilter(settings.theme.accent)};
}

/* ================= Colors ====================*/

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

.text-warning{
	color: var(--warning);
}

.text-danger{
	color: var(--danger);
}

.hover-bg-accent:hover{
	background-color: var(--accent);
}

.hover-bg-danger:hover{
	background-color: var(--danger);
}

.hover-text-on-accent:hover{
	color: var(--on-accent);
}

.hover-text-on-danger:hover{
    color: var(--on-danger);
}

/* =================== Components ================== */



.card{
    background-color: var(--secondary);
    border: 1px solid var(--tertiary);
    padding: 8px;
	border-radius: 14px;
}

.warning{
    border: 1px solid var(--warning);
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

.icon-size{
	height: var(--result-icon-size);
	width: var(--result-icon-size);
}

.search-icon-size{
    height: var(--search-icon-size);
    width: var(--search-icon-size);
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

.result-confirm{
	height: var(--result-confirm-size);
}

.one-line{
	min-width: 0;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
}

</style>
    `;
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
