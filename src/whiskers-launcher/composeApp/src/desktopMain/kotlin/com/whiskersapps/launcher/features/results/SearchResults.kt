package com.whiskersapps.launcher.features.results

import kotlinx.serialization.Serializable

@Serializable
data class SearchResults(
    val viewType: String = "",
    val results: List<SearchResult> = emptyList()
)

@Serializable
data class SearchResult(
    val icon: String? = null,
    val iconTint: String? = null,
    val title: String = "",
    val description: String? = null,
    val action: ResultAction = ResultAction()
)

@Serializable
data class ResultAction(
    val actionType: String = "",
    val dangerous: Boolean = false,
    val copyTextAction: CopyTextAction? = null,
    val copyImageAction: CopyImageAction? = null,
    val openLinkAction: OpenLinkAction? = null,
    val openAppAction: OpenAppAction? = null,
    val openFormAction: OpenFormAction? = null,
    val runExtensionAction: RunExtensionAction? = null
)

@Serializable
data class CopyTextAction(
    val text: String = ""
)

@Serializable
data class CopyImageAction(
    val imagePath: String = ""
)

@Serializable
data class OpenLinkAction(
    val link: String = ""
)

@Serializable
data class OpenAppAction(
    val appId: String = ""
)

@Serializable
data class OpenFormAction(
    val extensionId: String = "",
    val command: String = "",
    val fields: List<FormField> = emptyList(),
    val args: List<String> = emptyList()
)

@Serializable
data class RunExtensionAction(
    val extensionId: String = "",
    val command: String = "",
    val args: List<String> = emptyList()
)

@Serializable
data class FormField(
    val id: String = "",
    val fieldType: String = "",
    val inputField: FormInputField? = null,
    val textAreaField: FormTextAreaField? = null,
    val toggleField: FormToggleField? = null,
    val selectField: FormSelectField? = null,
    val filePickerField: FormFilePickerField? = null,
    val folderPickerField: FormFolderPickerField? = null
)

@Serializable
data class FormInputField(
    val title: String = "",
    val description: String = "",
    val text: String = "",
    val placeholder: String = "",
    val validation: String? = null
)

@Serializable
data class FormTextAreaField(
    val title: String = "",
    val description: String = "",
    val text: String = "",
    val placeholder: String = "",
    val validation: String? = null
)

@Serializable
data class FormToggleField(
    val title: String = "",
    val description: String = "",
    val toggled: Boolean = false
)

@Serializable
data class FormSelectField(
    val title: String = "",
    val description: String = "",
    val selectedOptionId: String = "",
    val options: List<FormSelectOption> = emptyList()
)

@Serializable
data class FormSelectOption(
    val id: String = "",
    val text: String = ""
)

@Serializable
data class FormFilePickerField(
    val title: String = "",
    val description: String = "",
    val filePath: String = "",
    val fileTypes: List<String>? = null,
    val validation: String? = null
)

@Serializable
data class FormFolderPickerField(
    val title: String = "",
    val description: String = "",
    val folderPath: String? = null,
    val validation: String? = null
)