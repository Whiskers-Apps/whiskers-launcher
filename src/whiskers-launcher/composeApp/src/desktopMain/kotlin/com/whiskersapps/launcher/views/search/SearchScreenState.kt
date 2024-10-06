package com.whiskersapps.launcher.views.search

import com.whiskersapps.launcher.features.results.SearchResult
import com.whiskersapps.launcher.features.settings.SettingsRepository

data class SearchScreenState(
    val loading: Boolean = true,
    val searchText: String = "",
    val resultsType: String = "",
    val results: List<SearchResult> = emptyList(),
    val totalResultsCount: Int = 0,
    val selectedIndex: Int = 0,
    val resultOffset: Int = 0,
    val askConfirmation: Boolean = false,
    val closeWindow: Boolean = false,
    val settings: SettingsRepository.Companion.Settings = SettingsRepository.Companion.Settings(),
    val theme: SettingsRepository.Companion.ComposedTheme = SettingsRepository.Companion.ComposedTheme()
)
