export interface SearchResults{
    view_type: string,
    results: SearchResult[]   
}

export interface SearchResult{
    icon: string | null,
    icon_tint: string | null,
    title: string,
    description: string | null,
    action: ResultAction
}

export interface ResultAction{
    action_type: string,
    dangerous: boolean,
    copy_text_action: CopyTextAction | null,
    copy_image_action: CopyImageAction | null,
    open_link_action: OpenLinkAction | null,
    open_app_action: OpenAppAction | null,
    open_form_action: OpenFormAction | null,
    run_extension_action: RunExtensionAction | null
}

export interface CopyTextAction{
    text: string
}

export interface CopyImageAction{
    image_path: string
}

export interface OpenLinkAction{
    link: string
}

export interface OpenAppAction{
    app_id: string
}

export interface OpenFormAction{
    extension_id: string,
    command: string,
    title: string,
    fields: FormField[],
    args: string[],
    action_text: string
}

export interface FormField{
    id: string,
    field_type: string,
    args: string[],
    input_field: FormInputField | null,
    text_area_field: FormTextAreaField | null,
    toggle_field: FormToggleField | null,
    select_field: FormSelectField | null,
    file_picker_field: FormFilePickerField | null,
    folder_picker_field: FormFolderPickerField | null,
}

export interface FormInputField{
    title: string,
    description: string,
    text: string,
    placeholder: string,
    validation: string[] | null
}

export interface FormTextAreaField{
    title: string,
    description: string,
    text: string,
    placeholder: string,
    validation: string | null
}

export interface FormToggleField{
    title: string,
    description: string,
    toggled: boolean
}

export interface FormSelectField{
    title: string,
    description: string,
    selected_option_id: string,
    options: FormSelectOption[]
}

export interface FormSelectOption{
    id: string,
    text: string
}

export interface FormFilePickerField{
    title: string,
    description: string,
    file_path: string | null,
    file_types: string[] | null,
    validation: string | null
}

export interface FormFolderPickerField{
    title: string,
    description: string,
    folder_path: string | null,
    validation: string | null
}

export interface RunExtensionAction{
    extension_id: string,
    command: string,
    args: string[]
}
