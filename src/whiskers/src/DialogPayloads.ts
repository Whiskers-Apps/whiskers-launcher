export interface DeleteSearchEnginePayload{
    index: number
}

export interface AddToBlacklistPayload{
    paths: string[]
}

export interface SearchEnginePayload{
    icon_path: string | null,
    tint_icon: boolean,
    keyword: string,
    query: string,
    name: string,
    default: boolean
}

export interface CloneExtensionPayload{
    url: string
}