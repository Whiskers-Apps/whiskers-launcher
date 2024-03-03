import { hexToCSSFilter } from "hex-to-css-filter";
import { Settings, Theme } from "@pages/Settings/ViewModel";
import { invoke } from "@tauri-apps/api";

export function getIconUrl(path: string): string {
  return new URL(`./assets/icons/${path}`, import.meta.url).href;
}

export function getHexCssFilter(hexColor: string): string {
  const filter = hexToCSSFilter(hexColor, {
    acceptanceLossPercentage: 0,
  });
  return filter.filter.replace(";", "");
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
