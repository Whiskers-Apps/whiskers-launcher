import { invoke } from "@tauri-apps/api"



export interface SimpleKlResult {
    icon?: string,
    icon_color?: string,
    title?: string,
    text?: string,
    description?: string,
    action?: OpenAppAction | OpenInBrowserAction | CopyToClipboardAction | ExtensionAction | DialogAction | DoNothingAction
}


export interface ResultAction {
    OpenApp?: OpenAppAction,
    OpenInBrowser?: OpenInBrowserAction,
    CopyToClipboard?: CopyToClipboardAction,
    ExtensionAction?: ExtensionAction,
    DialogAction?: DialogAction,
    DoNothingAction?: DoNothingAction
}

export interface OpenAppAction {
    type: string,
    desktop_path: string,
}

export interface OpenInBrowserAction {
    type: string,
    url: string,
}

export interface CopyToClipboardAction {
    type: string,
    text: string,
}

export interface ExtensionAction {
    type: string,
    action: string,
    args?: string[]
}

export interface DialogAction {
    extension_id: string,
    title: string,
    type: string,
    action: string,
    button_text: string,
    fields: DialogField[]
}

export interface DialogField {
    type: string,
    id: string,
    value: string | boolean,
    default_value: string,
    title?: string,
    description?: string,
    placeholder?: string,
    options?: SelectOption[] | CheckOption[]
}

export interface DialogResult{
    extension_id: string,
    action: string,
    results: DialogFieldResult[]
}

export interface DialogFieldResult{
    field_id: string,
    value: string
}

export interface SelectOption {
    id: string,
    value: string
}

export interface CheckOption{
    id: string,
    title: string,
    description: string,
    checked: boolean
}

export interface DoNothingAction {
    type: string,
}

export interface ExtensionManifest {
    id: string,
    version: string,
    name: string,
    description: string,
    icon: string,
    os: string,
    keyword: string,
    settings: ExtensionSettings
}

export interface ExtensionSettings {
    any?: ExtensionSetting[],
    linux?: ExtensionSetting[],
    windows?: ExtensionSetting[],
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

export interface ExtensionOption {
    name: string,
    value: string
}

export interface ExtensionShowCondition {
    setting: string,
    value: string
}

export async function getExtensions(): Promise<ExtensionManifest[]> {
    return JSON.parse(await invoke("get_extensions_json"))
}