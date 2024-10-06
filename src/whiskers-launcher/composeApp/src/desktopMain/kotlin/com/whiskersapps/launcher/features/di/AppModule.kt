package com.whiskersapps.launcher.features.di

import com.whiskersapps.launcher.features.settings.SettingsRepository
import com.whiskersapps.launcher.views.search.SearchScreenVM
import org.koin.core.module.Module
import org.koin.core.module.dsl.viewModelOf
import org.koin.dsl.module

val appModule : Module = module{
    single { SettingsRepository() }
    viewModelOf(::SearchScreenVM)
}
