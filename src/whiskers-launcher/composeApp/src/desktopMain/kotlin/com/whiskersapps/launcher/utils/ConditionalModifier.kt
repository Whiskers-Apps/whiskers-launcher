package com.whiskersapps.launcher.utils

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

@Composable
fun Modifier.modifyWhen(condition: Boolean, modifier: @Composable Modifier.() -> Modifier): Modifier{
    return if(condition) then(modifier()) else this
}