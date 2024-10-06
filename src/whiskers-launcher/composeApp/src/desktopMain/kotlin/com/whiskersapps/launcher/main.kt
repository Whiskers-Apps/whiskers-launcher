package com.whiskersapps.launcher

import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.WindowState
import androidx.compose.ui.window.application
import com.whiskersapps.launcher.features.di.appModule
import com.whiskersapps.launcher.features.settings.SettingsRepository
import com.whiskersapps.launcher.views.search.SearchScreen
import org.koin.compose.KoinContext
import org.koin.compose.koinInject
import org.koin.core.context.startKoin

fun main() = application {

    startKoin {
        modules(appModule)
    }

    val settingsRepository = koinInject<SettingsRepository>()
    val settings = settingsRepository.settings.collectAsState().value

    Window(
        title = "Whiskers Launcher",
        onCloseRequest = ::exitApplication,
        resizable = false,
        alwaysOnTop = true,
        undecorated = true,
        state = WindowState(
            size = DpSize(900.dp, 700.dp),
            position = WindowPosition(Alignment.Center),
        )
    ) {
        if (settings != null) {
            SearchScreen()
        }
    }
}