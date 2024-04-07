import { Settings, Theme } from "@pages/Settings/ViewModel";
import { invoke } from "@tauri-apps/api";
import CssFilterConverter from "css-filter-converter";

export function getIconUrl(path: string): string {
  return new URL(`./assets/icons/${path}`, import.meta.url).href;
}

export function getHexCssFilter(hexColor: string): string {
  let loss = 0;
  let attempts = 0;
  let filter = "";

  do {
    let result = CssFilterConverter.hexToFilter(hexColor);
    loss = result.loss ?? 0;
    filter = result.color ?? "";
    attempts += 1;
  } while (loss > 0.1 || attempts > 100);
  
  return filter;
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
