package com.whiskersapps.launcher.utils

import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.toComposeImageBitmap
import androidx.compose.ui.res.loadSvgPainter
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonNamingStrategy
import java.io.File
import java.io.FileInputStream
import javax.imageio.ImageIO
import javax.swing.Painter

@OptIn(ExperimentalSerializationApi::class)
val jsonConverter = Json {
    namingStrategy = JsonNamingStrategy.SnakeCase
}


@OptIn(ExperimentalSerializationApi::class)
val jsonRunnerConverter = Json {
    namingStrategy = JsonNamingStrategy.SnakeCase
    encodeDefaults = true
}


inline fun <reified T> getFromCompanion(args: List<String>): T {
    val command = ArrayList<String>().apply {
        add("whiskers-launcher-companion")
        addAll(args)
    }

    val process = ProcessBuilder(command).start()
    val output = process.inputStream.bufferedReader().readText()

    return jsonConverter.decodeFromString(output)
}

inline fun <reified T> getFromCompanion(arg: String): T {
    val command = listOf("whiskers-launcher-companion", arg)

    val process = ProcessBuilder(command).start()
    val output = process.inputStream.bufferedReader().readText()

    return jsonConverter.decodeFromString(output)
}

enum class ImageType {
    Svg,
    Image
}

fun getImageType(path: String): ImageType = if (path.endsWith(".svg")) ImageType.Svg else ImageType.Image


fun getLocalImage(path: String): ImageBitmap? {
    return try {
        val file = File(path)
        val image = ImageIO.read(file)
        image?.toComposeImageBitmap()
    } catch (e: Exception) {
        println("Error getting image: $e")
        null
    }
}

fun getSvgStream(path: String): FileInputStream = File(path).inputStream()