package com.whiskersapps.launcher.views.search

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.whiskersapps.launcher.features.results.SearchResults
import com.whiskersapps.launcher.features.settings.SettingsRepository
import com.whiskersapps.launcher.features.theming.getComposedTheme
import com.whiskersapps.launcher.utils.getFromCompanion
import com.whiskersapps.launcher.utils.jsonConverter
import com.whiskersapps.launcher.utils.jsonRunnerConverter
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class SearchScreenVM(private val settingsRepository: SettingsRepository) : ViewModel() {

    private val _state = MutableStateFlow(SearchScreenState())
    val state = _state.asStateFlow()

    private var searchResults: SearchResults = SearchResults()

    init {
        viewModelScope.launch(Dispatchers.IO) {
            settingsRepository.settings.collect { settings ->
                _state.update {
                    it.copy(
                        loading = false,
                        settings = settings!!,
                        theme = settings.theme.getComposedTheme()
                    )
                }

                onSearch()
            }
        }
    }

    fun onAction(action: SearchScreenAction) {
        when (action) {
            is SearchScreenAction.OnSearchInput -> {
                onSearchInput(action.text)
            }

            SearchScreenAction.OnKeyDown -> onKeyDown()
            SearchScreenAction.OnKeyLeft -> onKeyLeft()
            SearchScreenAction.OnKeyRight -> onKeyRight()
            SearchScreenAction.OnKeyUp -> onKeyUp()
            SearchScreenAction.OnOpenSettings -> onOpenSettings()
            SearchScreenAction.OnRunAction -> onRunAction()
        }
    }

    private fun onSearchInput(text: String) {
        _state.update { it.copy(searchText = text) }
        onSearch()
    }

    private fun onKeyDown() {
        goToNextResult()
    }

    private fun onKeyUp() {
        goToPreviousResult()
    }

    private fun onKeyRight() {
    }

    private fun onKeyLeft() {
    }

    private fun onOpenSettings() {

    }

    private fun onRunAction() {
        val selectedIndex = state.value.selectedIndex
        val askConfirmation = state.value.askConfirmation
        val result = state.value.results[selectedIndex]

        if (!askConfirmation) {
            if (result.action.dangerous) {
                _state.update { it.copy(askConfirmation = true) }
            } else {
                executeAction()
            }
        } else {
            _state.update { it.copy(askConfirmation = false) }
            executeAction()
        }
    }

    private fun executeAction() {
        viewModelScope.launch(Dispatchers.IO) {
            val action = state.value.results[state.value.selectedIndex].action
            val actionJson = jsonRunnerConverter.encodeToString(action)

            val command = listOf("whiskers-launcher-companion", "run-action", actionJson)

            println(actionJson)

            ProcessBuilder(command).start()

            _state.update { it.copy(closeWindow = true) }
        }
    }

    private fun onSearch() {
        viewModelScope.launch(Dispatchers.IO) {
            searchResults = getFromCompanion(listOf("search", state.value.searchText))

            _state.update {
                it.copy(
                    selectedIndex = 0,
                    resultOffset = 0,
                    resultsType = searchResults.viewType,
                    totalResultsCount = searchResults.results.size
                )
            }

            sliceResults()
        }
    }

    private fun sliceResults() {
        val maxOffset =
            if (state.value.resultOffset + 9 > searchResults.results.size) searchResults.results.size else state.value.resultOffset + 9

        _state.update {
            it.copy(
                results = searchResults.results.subList(it.resultOffset, maxOffset)
            )
        }
    }

    private fun goToNextResult() {
        val selectedIndex = state.value.selectedIndex
        val offset = state.value.resultOffset
        val displayedResults = state.value.results

        _state.update { it.copy(askConfirmation = false) }

        if (selectedIndex < displayedResults.size - 1) {
            _state.update { it.copy(selectedIndex = selectedIndex + 1) }
            return
        }

        if (offset + selectedIndex < searchResults.results.size - 1) {
            _state.update { it.copy(resultOffset = offset + 1) }
            sliceResults()
            return
        }

        if (searchResults.results.size < 9) {
            if (selectedIndex + 1 == searchResults.results.size) {
                _state.update { it.copy(selectedIndex = 0) }
                return
            }
        }

        _state.update {
            it.copy(
                resultOffset = 0,
                selectedIndex = 0
            )
        }

        sliceResults()
    }

    private fun goToPreviousResult() {
        val selectedIndex = state.value.selectedIndex
        val offset = state.value.resultOffset

        _state.update { it.copy(askConfirmation = false) }

        if (selectedIndex > 0) {
            _state.update { it.copy(selectedIndex = selectedIndex - 1) }
            return
        }

        if (offset - 1 > 0) {
            _state.update { it.copy(resultOffset = offset - 1) }
            sliceResults()
            return
        }

        if (offset == 0) {
            if (searchResults.results.size < 9) {
                _state.update { it.copy(selectedIndex = selectedIndex - 1) }
                return
            }
        }

        _state.update {
            it.copy(
                resultOffset = searchResults.results.size - 9,
                selectedIndex = 8
            )
        }

        sliceResults()
    }

    private fun onSelectAltResult(index: Int) {
        val displayedResults = state.value.results

        if (index > displayedResults.size) {
            return
        }

        _state.update { it.copy(selectedIndex = index) }

        onRunAction()
    }
}