package com.whiskersapps.launcher.features.theming

import androidx.compose.ui.graphics.Color
import com.whiskersapps.launcher.features.settings.SettingsRepository
import com.whiskersapps.launcher.features.settings.SettingsRepository.Companion.ComposedTheme

fun SettingsRepository.Companion.Theme.getComposedTheme(): ComposedTheme {
    return ComposedTheme(
        background = Color(this.background.asHexLong()),
        secondary = Color(this.secondary.asHexLong()),
        tertiary = Color(this.tertiary.asHexLong()),
        accent = Color(this.accent.asHexLong()),
        warning = Color(this.warning.asHexLong()),
        danger = Color(this.danger.asHexLong()),
        onAccent = Color(this.onAccent.asHexLong()),
        onDanger = Color(this.onDanger.asHexLong()),
        text = Color(this.text.asHexLong()),
        subText = Color(this.subText.asHexLong())
    )
}

fun String.asHexLong(): Long {
    return java.lang.Long.decode(this.replace("#", "#ff"))
}