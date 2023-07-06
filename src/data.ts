import { invoke } from "@tauri-apps/api"

export interface SimpleKlResult {
    Text?: TextResult,
    IconWithText?: IconWithTextResult,
    TitleAndDescription?: TitleAndDescriptionResult,
    IconWithTitleAndDescription?: IconWithTitleAndDescriptionResult
}

export interface TextResult {
    text: string,
    action: ResultAction
}

export interface IconWithTextResult {
    icon: string,
    text: string,
    action: ResultAction
}

export interface TitleAndDescriptionResult {
    title: string,
    description: string,
    action: ResultAction
}

export interface IconWithTitleAndDescriptionResult {
    icon: string,
    title: string,
    description: string,
    action: ResultAction
}

export interface ResultAction {
    OpenApp?: OpenAppAction,
    OpenInBrowser?: OpenInBrowserAction,
    CopyToClipboard?: CopyToClipboardAction,
    ExtensionAction?: ExtensionAction
}

export interface OpenAppAction {
    desktop_path: string,
}

export interface OpenInBrowserAction {
    url: string,
}

export interface CopyToClipboardAction {
    text: string,
}

export interface ExtensionAction {
    action: string,
    args?: string[]
}

export interface ExtensionManifest {
    id: string,
    name: string,
    description: string,
    icon: string,
    os: string,
    keyword: string,
    settings: ExtensionSettings[]
}

export interface ExtensionSettings {
    any: ExtensionSetting[],
    linux: ExtensionSetting[],
    windows: ExtensionSetting[],
}

export interface ExtensionSetting {
    id: string,
    name: string,
    description?: string,
    input: string,
    default_value: string,
    options?: ExtensionOption[],
    show_condition?: ExtensionShowCondition[] 
}

export interface ExtensionOption{
    name: string,
    value: string
}

export interface ExtensionShowCondition{
    setting: string,
    value: string
}

export async function getExtensions(): Promise<ExtensionManifest[]>{
    return JSON.parse(await invoke("get_extensions_json"))
}