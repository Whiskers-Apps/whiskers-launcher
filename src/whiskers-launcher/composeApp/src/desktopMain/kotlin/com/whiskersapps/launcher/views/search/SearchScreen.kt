package com.whiskersapps.launcher.views.search

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.isCtrlPressed
import androidx.compose.ui.input.key.key
import androidx.compose.ui.input.key.onKeyEvent
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.loadSvgPainter
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.whiskersapps.launcher.features.theming.asHexLong
import com.whiskersapps.launcher.utils.ImageType
import com.whiskersapps.launcher.utils.getImageType
import com.whiskersapps.launcher.utils.getLocalImage
import com.whiskersapps.launcher.utils.getSvgStream
import com.whiskersapps.launcher.utils.modifyWhen
import org.koin.compose.KoinContext
import org.koin.compose.viewmodel.koinViewModel
import java.io.File
import kotlin.system.exitProcess

@Composable
fun SearchScreen() {
    KoinContext {
        val vm = koinViewModel<SearchScreenVM>()
        val state = vm.state.collectAsState().value
        val focusRequester = remember { FocusRequester() }

        if(state.closeWindow){
            exitProcess(0)
        }

        if (!state.loading) {
            LaunchedEffect(Unit) {
                focusRequester.requestFocus()
            }

            Column(
                modifier = Modifier.fillMaxSize()
                    .background(state.theme.background)
                    .padding(16.dp)
                    .onKeyEvent { event ->
                        if (event.isCtrlPressed && event.key == Key.S) {
                            vm.onAction(SearchScreenAction.OnOpenSettings)
                            true
                        } else {
                            when (event.key) {
                                Key.DirectionDown -> {
                                    vm.onAction(SearchScreenAction.OnKeyDown)
                                    true
                                }

                                Key.DirectionUp -> {
                                    vm.onAction(SearchScreenAction.OnKeyUp)
                                    true
                                }

                                Key.DirectionRight -> {
                                    vm.onAction(SearchScreenAction.OnKeyRight)
                                    true
                                }

                                Key.DirectionLeft -> {
                                    vm.onAction(SearchScreenAction.OnKeyLeft)
                                    true
                                }

                                Key.Enter -> {
                                    vm.onAction(SearchScreenAction.OnRunAction)
                                    true
                                }

                                else -> {
                                    false
                                }
                            }
                        }
                    }
                    .focusable(),
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth()
                        .clip(CircleShape)
                        .background(state.theme.secondary)
                        .padding(12.dp)
                ) {
                    if (state.settings.showSearchIcon) {
                        Icon(
                            modifier = Modifier.size(20.dp),
                            painter = androidx.compose.ui.res.painterResource("icons/search.svg"),
                            contentDescription = null,
                            tint = state.theme.text
                        )

                        Spacer(modifier = Modifier.width(8.dp))
                    }

                    Box(
                        modifier = Modifier.fillMaxWidth().weight(1f, fill = true),
                    ) {
                        BasicTextField(
                            modifier = Modifier
                                .fillMaxWidth()
                                .focusRequester(focusRequester),
                            value = state.searchText,
                            onValueChange = { text ->
                                vm.onAction(SearchScreenAction.OnSearchInput(text))
                            },
                            textStyle = TextStyle(
                                color = state.theme.text
                            ),
                            cursorBrush = Brush.verticalGradient(
                                colors = listOf(state.theme.text, state.theme.text)
                            ),
                            maxLines = 1
                        )

                        if (state.searchText.isEmpty() && state.settings.showPlaceholder) {
                            Text("Search apps, extensions, web", color = state.theme.subText)
                        }
                    }

                    if (state.settings.showSettingsIcon) {
                        Spacer(modifier = Modifier.width(8.dp))

                        Icon(
                            modifier = Modifier
                                .size(20.dp)
                                .clickable {

                                },
                            painter = androidx.compose.ui.res.painterResource("icons/settings.svg"),
                            contentDescription = null,
                            tint = state.theme.text
                        )
                    }
                }

                Spacer(modifier = Modifier.height(16.dp))

                if (state.results.isEmpty()) {
                    Column(
                        modifier = Modifier.fillMaxWidth().fillMaxHeight().weight(1f, fill = true),
                        verticalArrangement = Arrangement.Center,
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Icon(
                            modifier = Modifier.size(80.dp),
                            painter = androidx.compose.ui.res.painterResource("icons/cat.svg"),
                            contentDescription = null,
                            tint = state.theme.text
                        )

                        Text("This is kinda empty \uD83E\uDEE4", color = state.theme.text)
                    }
                } else {

                    if (state.resultsType == "List") {
                        LazyColumn(
                            modifier = Modifier.fillMaxWidth().fillMaxHeight()
                                .weight(1f, fill = true)
                        ) {
                            itemsIndexed(
                                items = state.results,
                                key = { index, _ -> "result-$index" }
                            ) { index, result ->

                                val isSelected = index == state.selectedIndex

                                Row(
                                    modifier = Modifier.fillMaxWidth().clip(CircleShape)
                                        .modifyWhen(isSelected) {
                                            this.background(state.theme.secondary)
                                        }.padding(12.dp),
                                    verticalAlignment = Alignment.CenterVertically
                                ) {
                                    result.icon?.let { iconPath ->

                                        val colorFilter = if (result.iconTint != null) {
                                            if (result.iconTint == "accent") {
                                                ColorFilter.tint(state.theme.accent)
                                            } else {
                                                ColorFilter.tint(Color(result.iconTint.asHexLong()))
                                            }
                                        } else {
                                            null
                                        }


                                        when (getImageType(iconPath)) {
                                            ImageType.Svg -> {
                                                Image(
                                                    modifier = Modifier.size(32.dp),
                                                    painter = loadSvgPainter(
                                                        getSvgStream(iconPath),
                                                        density = LocalDensity.current
                                                    ),
                                                    contentDescription = null,
                                                    colorFilter = colorFilter
                                                )
                                            }

                                            ImageType.Image -> {
                                                getLocalImage(iconPath)?.let { icon ->
                                                    Image(
                                                        modifier = Modifier.size(32.dp),
                                                        bitmap = icon,
                                                        contentDescription = null,
                                                        colorFilter = colorFilter
                                                    )
                                                }
                                            }
                                        }

                                    }

                                    Spacer(modifier = Modifier.width(16.dp))

                                    Column {
                                        Text(
                                            result.title,
                                            color = if (isSelected) state.theme.accent else state.theme.text,
                                            overflow = TextOverflow.Ellipsis,
                                            maxLines = 1
                                        )
                                        result.description?.let { description ->

                                            Text(
                                                description,
                                                color = state.theme.text,
                                                overflow = TextOverflow.Ellipsis,
                                                maxLines = 1
                                            )
                                        }

                                    }
                                }
                            }
                        }
                    } else {
                        Text("grid", color = state.theme.text)

                    }
                }

                if (state.results.isNotEmpty()) {
                    Box(modifier = Modifier.fillMaxWidth(), contentAlignment = Alignment.Center) {
                        Text(
                            "Total Results ${state.totalResultsCount}",
                            color = state.theme.text,
                            overflow = TextOverflow.Ellipsis,
                            maxLines = 1
                        )
                    }
                }
            }
        }
    }
}