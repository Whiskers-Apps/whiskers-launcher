package com.whiskersapps.launcher.views.search

sealed class SearchScreenAction {
    data class OnSearchInput(val text: String) : SearchScreenAction()
    data object OnKeyDown: SearchScreenAction()
    data object OnKeyUp: SearchScreenAction()
    data object OnKeyRight: SearchScreenAction()
    data object OnKeyLeft: SearchScreenAction()
    data object OnOpenSettings: SearchScreenAction()
    data object OnRunAction: SearchScreenAction()
}