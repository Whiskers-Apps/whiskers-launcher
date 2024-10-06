import org.jetbrains.compose.desktop.application.dsl.TargetFormat

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.jetbrainsCompose)
    alias(libs.plugins.compose.compiler)
    kotlin("plugin.serialization") version "2.0.20"
}

kotlin {
    jvm("desktop")

    linuxX64{
        binaries {
            executable("whiskers-launcher", listOf(RELEASE)){
                baseName = "whiskers-launcher"
            }
        }
    }

    sourceSets {
        val desktopMain by getting
        
        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material)
            implementation(compose.ui)
            implementation(compose.components.uiToolingPreview)
            implementation(libs.kotlinx.coroutines.swing)

            api(libs.koin.core)
            implementation(libs.koin.compose)
            implementation(libs.koin.composeVM)

            implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.1")
        }
        desktopMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(compose.components.resources)
        }
    }
}

compose.desktop {
    application {
        mainClass = "com.whiskersapps.launcher.MainKt"

        nativeDistributions {
            targetFormats(TargetFormat.Exe, TargetFormat.Deb)
            packageName = "com.whiskersapps.launcher"
            packageVersion = "3.0.0"
        }
    }
}

tasks.withType<Wrapper> {
    gradleVersion = "8.7"
    distributionType = Wrapper.DistributionType.BIN
}