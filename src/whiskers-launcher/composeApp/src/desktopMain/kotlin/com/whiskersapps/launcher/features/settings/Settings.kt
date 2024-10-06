package com.whiskersapps.launcher.features.settings

import androidx.compose.ui.graphics.Color
import com.whiskersapps.launcher.features.settings.SettingsRepository.Companion.ComposedTheme
import com.whiskersapps.launcher.utils.getFromCompanion
import com.whiskersapps.launcher.utils.jsonConverter
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.serialization.Serializable

class SettingsRepository {

    companion object {
        @Serializable
        data class Settings(
            val firstKey: String = "",
            val secondKey: String? = null,
            val thirdKey: String = "",
            val autoStart: Boolean = true,
            val showRecentApps: Boolean = true,
            val showSearchIcon: Boolean = true,
            val showPlaceholder: Boolean = true,
            val showSettingsIcon: Boolean = true,
            val showAltHint: Boolean = true,
            val blacklist: List<String> = emptyList(),
            val searchKeyword: String = "s",
            val searchEngines: List<SearchEngine> = emptyList(),
            val defaultSearchEngine: Int = -1,
            val theme: Theme = Theme(),
            val extensions: List<ExtensionSetting> = emptyList(),
        )

        @Serializable
        data class SearchEngine(
            val id: Int = -1,
            val iconPath: String? = "",
            val tintIcon: Boolean = false,
            val keyword: String = "",
            val name: String = "",
            val searchQuery: String = ""
        )

        @Serializable
        data class Theme(
            val background: String = "",
            val secondary: String = "",
            val tertiary: String = "",
            val accent: String = "",
            val warning: String = "",
            val danger: String = "",
            val onAccent: String = "",
            val onDanger: String = "",
            val text: String = "",
            val subText: String = "",
        )

        @Serializable
        data class ExtensionSetting(
            val extensionId: String = "",
            val settingId: String = "",
            val settingValue: String = ""
        )

        data class ComposedTheme(
            val background: Color = Color.Transparent,
            val secondary: Color = Color.Transparent,
            val tertiary: Color = Color.Transparent,
            val accent: Color = Color.Transparent,
            val warning: Color = Color.Transparent,
            val danger: Color = Color.Transparent,
            val onAccent: Color = Color.Transparent,
            val onDanger: Color = Color.Transparent,
            val text: Color = Color.Transparent,
            val subText: Color = Color.Transparent,
        )
    }

    private val _settings = MutableStateFlow<Settings?>(null)
    val settings = _settings.asStateFlow()

    init {
        val settings: Settings = getFromCompanion("get-settings")
        _settings.update { settings }
    }
}