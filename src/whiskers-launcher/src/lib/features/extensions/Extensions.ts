export interface ExtensionManifest {
	id: string;
	name: string;
	description: string;
	keyword: string[];
	settings: ExtensionManifestSetting[] | null;
	os: string;
}

export interface ExtensionManifestSetting {
	id: string;
	title: string;
	description: string;
	setting_type: string;
	default_value: string;
	show_conditions: ExtensionManifestShowCondition[] | null;
	select_options: ExtensionManifestSelectOption[] | null;
	os: string;
}

export interface ExtensionManifestShowCondition {
	setting_id: string;
	setting_value: string;
}

export interface ExtensionManifestSelectOption{
    id: string,
    text: string
}

