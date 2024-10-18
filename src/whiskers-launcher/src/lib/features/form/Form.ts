export interface FormResponse{
    results: FormResult[],
    args: string[]
}

export interface FormResult{
    field_id: string,
    field_value: string,
    args: string[]
}