import { Settings, Theme } from "@pages/Settings/ViewModel";
import { invoke } from "@tauri-apps/api";
import CssFilterConverter from "css-filter-converter";
import { hexToCSSFilter } from "hex-to-css-filter";

export function getIconUrl(path: string): string {
  return new URL(`./assets/icons/${path}`, import.meta.url).href;
}

export function getHexCssFilter(hexColor: string): string {
  const { filter } = hexToCSSFilter(hexColor, {
    acceptanceLossPercentage: 0.1,
    maxChecks: 50,
  });

  return filter.slice(0, -1);
}

export async function getSettings(): Promise<Settings> {
  return await invoke("get_settings");
}

export async function getTheme(): Promise<Theme> {
  return (await getSettings()).theme;
}

export function getScaledPixels(scale: number, pixels: number): string {
  return `${scale * pixels}px`;
}

export function getScaledSize(scale: number, pixels: number): number {
  return scale * pixels;
}
