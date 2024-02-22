import { invoke } from "@tauri-apps/api";
import { platform } from "@tauri-apps/api/os";
import { SelectOption } from "@components/ComponentClasses";
import { emit, listen } from "@tauri-apps/api/event";
import { SearchEnginePayload, DeleteSearchEnginePayload } from "@/DialogPayloads";
import { WebviewWindow } from "@tauri-apps/api/window";
import { save, open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { getSettings } from "@/utils";

export class ViewModel {
  hasLoaded = false;
  settings: Settings | undefined = undefined;
  userExtensions: ExtensionManifest[] = [];
  userExtensionsDefaultValues: ExtensionDefaultValues[] = [];
  selectedTab: SettingsTab = SettingsTab.General;
  showNavbar = true;

  //General Tab
  launchFirstKeyOptions = LAUNCH_FIRST_KEY_OPTIONS;
  launchSecondKeyOptions = LAUNCH_SECOND_KEY_OPTIONS;
  launchThirdKeyOptions = LAUNCH_THIRD_KEY_OPTIONS;
  launchShortcutChanged = false;

  async load() {
    this.settings = await invoke("get_settings");

    const os = await platform();

    if (os === "win32") {
      this.launchFirstKeyOptions.filter((key) => key.id !== "super");
      this.launchSecondKeyOptions.filter((key) => key.id !== "super");
    }

    this.loadUserExtensions();

    await listen("load-settings", async (_event) => {
      this.settings = await invoke("get_settings");
      emit("load-theme");
    });
  }

  async loadUserExtensions() {
    await invoke("index_extensions");
    this.settings = await getSettings();
    this.userExtensions = await invoke("get_user_extensions");
    this.userExtensionsDefaultValues = await invoke("get_extensions_default_values");
  }

  getTabClasses(tab: SettingsTab): string {
    return `tab ${
      tab === this.selectedTab ? "tab-selected background-primary text-on-primary" : ""
    }`;
  }

  openTab(tab: SettingsTab) {
    this.selectedTab = tab;
  }

  async updateSettings(newSettings: Settings) {
    invoke("update_settings", { settings: newSettings });
    this.settings = newSettings;
  }

  updateLaunchFirstKey(key: string) {
    let newSettings = this.settings!!;
    newSettings.launch_first_key = key;

    this.updateSettings(newSettings);

    this.launchShortcutChanged = true;
  }

  updateLaunchSecondKey(key: string) {
    let newSettings = this.settings!!;
    newSettings.launch_second_key = key === "-" ? null : key;

    this.updateSettings(newSettings);

    this.launchShortcutChanged = true;
  }

  updateLaunchThirdKey(key: string) {
    let newSettings = this.settings!!;
    newSettings.launch_third_key = key;

    this.updateSettings(newSettings);

    this.launchShortcutChanged = true;
  }

  updateAutoStart(value: boolean) {
    let newSettings = this.settings!!;
    newSettings.auto_start = value;

    this.updateSettings(newSettings);
  }

  updateHideOnBlur(value: boolean) {
    let newSettings = this.settings!!;
    newSettings.hide_on_blur = value;

    this.updateSettings(newSettings);
  }

  updateFractionalScaling(value: number) {
    let newSettings = this.settings!!;
    newSettings.fractional_scaling = value;

    this.updateSettings(newSettings);
  }

  // Search Box Tab
  updateShowSearchIcon(value: boolean) {
    let newSettings = this.settings!!;
    newSettings.show_search_icon = value;

    this.updateSettings(newSettings);
  }

  updateShowSettingsIcon(value: boolean) {
    let newSettings = this.settings!!;
    newSettings.show_settings_icon = value;

    this.updateSettings(newSettings);
  }

  updateShowPlaceholder(value: boolean) {
    let newSettings = this.settings!!;
    newSettings.show_placeholder = value;

    this.updateSettings(newSettings);
  }

  updateLayout(value: string) {
    let newSettings = this.settings!!;
    newSettings.layout = value;

    this.updateSettings(newSettings);
  }

  updateBorderRadius(value: number) {
    let newSettings = this.settings!!;
    newSettings.border_radius = value;

    this.updateSettings(newSettings);
  }

  updateBorderWidth(value: number) {
    let newSettings = this.settings!!;
    newSettings.border_width = value;

    this.updateSettings(newSettings);
  }

  // Search Results Tab

  blacklistedApps: AppIndex[] = [];

  updateResultsCount(value: number) {
    let newSettings = this.settings!!;
    newSettings.results_count = value;

    this.updateSettings(newSettings);
  }

  updateDensity(value: string) {
    let newSettings = this.settings!!;
    newSettings.density = value;

    this.updateSettings(newSettings);
  }

  async loadBacklistedApps() {
    this.blacklistedApps = await invoke("get_blacklisted_apps");
  }

  addToBlacklist(path: string) {
    let newSettings = this.settings!!;
    let newBlacklist = newSettings.blacklist;

    newBlacklist.push(path);

    newSettings.blacklist = newBlacklist;

    this.updateSettings(newSettings);
  }

  async removeFromBlacklist(path: string) {
    let newSettings = this.settings!!;
    let newBlacklist = newSettings.blacklist;

    newBlacklist = newBlacklist.filter((p) => p !== path);

    newSettings.blacklist = newBlacklist;

    this.updateSettings(newSettings);
    await this.loadBacklistedApps();
  }

  // =========================================
  // Search Engines Page
  // =========================================

  toggleSearchEngineMenu(index: number) {
    let element = document.getElementById(`search-engine-menu-${index}`)!!;

    if (element.style.display == "block") {
      element.style.display = "none";
    } else {
      element.style.display = "block";
    }
  }

  makeDefaultSearchEngine(index: number) {
    this.closeSearchEngineMenu(index);

    const newSearchEngines: SearchEngine[] = [];

    this.settings!!.search_engines.forEach((se, seindex) => {
      const nse = se;
      nse.default = index === seindex;
      newSearchEngines.push(nse);
    });

    const newSettings = this.settings!!;
    newSettings.search_engines = newSearchEngines;

    this.updateSettings(newSettings);
  }

  async deleteSearchEngine(index: number) {
    this.closeSearchEngineMenu(index);

    new WebviewWindow("confirm-delete-search-engine", {
      url: `confirm-delete-search-engine?index=${index}`,
      title: "Delete Search Engine",
      resizable: false,
      width: 800,
      center: true,
      height: 200,
    });

    const unlisten = await listen<DeleteSearchEnginePayload>(
      "delete-search-engine",
      async (event) => {
        this.closeSearchEngineMenu(event.payload.index);

        const newSettings = this.settings!!;
        newSettings.search_engines = newSettings.search_engines.filter(
          (_, seindex) => seindex !== event.payload.index
        );

        this.updateSettings(newSettings);

        unlisten();
      }
    );
  }

  async addSearchEngine() {
    new WebviewWindow("add-search-engine", {
      url: "add-search-engine",
      title: "Add Search Engine",
      height: 800,
      width: 800,
      center: true,
    });

    const unlisten = await listen<SearchEnginePayload>("add-search-engine", (event) => {
      const newSearchEngine: SearchEngine = {
        icon_path: event.payload.icon_path,
        tint_icon: event.payload.tint_icon,
        name: event.payload.name,
        query: event.payload.query,
        keyword: event.payload.keyword,
        default: this.settings!!.search_engines.length === 0,
      };

      const newSearchEngines = this.settings!!.search_engines;
      newSearchEngines.push(newSearchEngine);

      const newSettings = this.settings!!;
      newSettings.search_engines = newSearchEngines;

      this.updateSettings(newSettings);

      unlisten();
    });
  }

  async editSearchEngine(index: number) {
    new WebviewWindow("edit-search-engine", {
      url: `edit-search-engine?index=${index}`,
      title: "Edit Search Engine",
      height: 800,
      width: 800,
      center: true,
    });

    const unlisten = await listen<SearchEnginePayload>("edit-search-engine", (event) => {
      const newSearchEngines: SearchEngine[] = [];

      this.settings!!.search_engines.forEach((se, seindex) => {
        if (seindex === index) {
          newSearchEngines.push({
            icon_path: event.payload.icon_path,
            tint_icon: event.payload.tint_icon,
            name: event.payload.name,
            query: event.payload.query,
            keyword: event.payload.keyword,
            default: event.payload.default,
          });
        } else {
          newSearchEngines.push(se);
        }
      });

      const newSettings = this.settings!!;
      newSettings.search_engines = newSearchEngines;

      this.updateSettings(newSettings);

      unlisten();
    });
  }

  closeSearchEngineMenu(index: number) {
    const div = document.getElementById(`search-engine-menu-${index}`)!!;
    div.style.display = "none";
  }

  // =============================
  // Theming Tab
  // =============================

  async importTheme() {
    const path = await open({
      defaultPath: await downloadDir(),
      filters: [{ name: "Json", extensions: ["json"] }],
    });

    if (path) {
      await invoke("import_theme", { path: path });
      this.settings = await invoke("get_settings");
      emit("load-theme");
    }
  }

  async exportTheme() {
    const path = await save({
      defaultPath: `${await downloadDir()}/CustomTheme.json`,
      filters: [{ name: "Json", extensions: ["json"] }],
    });

    if (path) {
      invoke("export_theme", { path: path });
    }
  }

  private updateTheme(newTheme: Theme) {
    const newSettings = this.settings!!;
    newSettings.theme = newTheme;

    this.updateSettings(newSettings);

    emit("load-theme");
  }

  updateBackgroundMain(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.background_main = hex;
    this.updateTheme(newTheme);
  }

  updateBackgroundSecondary(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.background_secondary = hex;
    this.updateTheme(newTheme);
  }

  updateBackgroundTertiary(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.background_tertiary = hex;
    this.updateTheme(newTheme);
  }

  updateAccentPrimary(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.accent_primary = hex;
    this.updateTheme(newTheme);
  }

  updateAccentDanger(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.accent_danger = hex;
    this.updateTheme(newTheme);
  }

  updateTextOnBackground(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.text_on_background = hex;
    this.updateTheme(newTheme);
  }

  updateTextOnPrimary(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.text_on_primary = hex;
    this.updateTheme(newTheme);
  }

  updateTextOnDanger(hex: string) {
    const newTheme = this.settings!!.theme;
    newTheme.text_on_danger = hex;
    this.updateTheme(newTheme);
  }
}

export interface Settings {
  launch_first_key: string;
  launch_second_key: string | null;
  launch_third_key: string;
  hide_on_blur: boolean;
  auto_start: boolean;
  fractional_scaling: number;
  show_search_icon: boolean;
  show_settings_icon: boolean;
  show_placeholder: boolean;
  layout: string;
  border_radius: number;
  border_width: number;
  results_count: number;
  density: string;
  blacklist: string[];
  search_engines: SearchEngine[];
  theme: Theme;
  extensions: Extension[];
}

export interface SearchEngine {
  icon_path: string | null;
  tint_icon: boolean;
  keyword: string;
  name: string;
  query: string;
  default: boolean;
}

export interface Theme {
  background_main: string;
  background_secondary: string;
  background_tertiary: string;
  accent_primary: string;
  accent_danger: string;
  text_on_background: string;
  text_on_primary: string;
  text_on_danger: string;
}

export interface Extension {
  id: string;
  keyword: string;
  settings: ExtensionSetting[];
}

export interface ExtensionSetting {
  id: string;
  value: string;
}

export enum SettingsTab {
  General = "/settings",
  SearchBox = "/search-box",
  SearchResults = "/search-results",
  SearchEngines = "/search-engines",
  Theming = "/theming",
  Extensions = "/extensions",
  About = "/about",
}

export const LAUNCH_FIRST_KEY_OPTIONS: SelectOption[] = [
  {
    id: "ctrl",
    text: "Ctrl",
  },
  {
    id: "alt",
    text: "Alt",
  },
  {
    id: "super",
    text: "Super",
  },
  {
    id: "shift",
    text: "Shift",
  },
];

export const LAUNCH_SECOND_KEY_OPTIONS: SelectOption[] = [
  {
    id: "-",
    text: "-",
  },
  {
    id: "alt",
    text: "Alt",
  },
  {
    id: "shift",
    text: "Shift",
  },
  {
    id: "super",
    text: "Super",
  },
];

export const LAUNCH_THIRD_KEY_OPTIONS: SelectOption[] = [
  {
    id: "space",
    text: "Space",
  },
  {
    id: "a",
    text: "A",
  },
  {
    id: "b",
    text: "B",
  },
  {
    id: "c",
    text: "C",
  },
  {
    id: "d",
    text: "D",
  },
  {
    id: "e",
    text: "E",
  },
  {
    id: "f",
    text: "F",
  },
  {
    id: "g",
    text: "G",
  },
  {
    id: "h",
    text: "H",
  },
  {
    id: "i",
    text: "I",
  },
  {
    id: "j",
    text: "J",
  },
  {
    id: "k",
    text: "K",
  },
  {
    id: "l",
    text: "L",
  },
  {
    id: "m",
    text: "M",
  },
  {
    id: "n",
    text: "N",
  },
  {
    id: "o",
    text: "O",
  },
  {
    id: "p",
    text: "P",
  },
  {
    id: "q",
    text: "Q",
  },
  {
    id: "r",
    text: "R",
  },
  {
    id: "s",
    text: "S",
  },
  {
    id: "t",
    text: "T",
  },
  {
    id: "u",
    text: "U",
  },
  {
    id: "v",
    text: "V",
  },
  {
    id: "w",
    text: "W",
  },
  {
    id: "x",
    text: "X",
  },
  {
    id: "y",
    text: "Y",
  },
  {
    id: "z",
    text: "Z",
  },
];

export const DENSITY_OPTIONS: SelectOption[] = [
  {
    id: "small",
    text: "Small",
  },
  {
    id: "medium",
    text: "Medium",
  },
  {
    id: "large",
    text: "Large",
  },
];

export interface AppIndex {
  icon_path: string | null;
  exec_path: string;
  name: string;
}

export interface ExtensionManifest {
  id: string;
  name: string;
  version_name: string;
  version_code: number;
  description: string;
  os: string[];
  keyword: string;
  settings: ExtensionSetting[];
}

export interface ExtensionSetting {
  id: string;
  title: string;
  description: string;
  setting_type: string;
  default_value: string;
  os: string[];
  select_options: ExtensionSelectOption[] | null;
  show_conditions: ExtensionShowCondition[] | null;
}

export interface ExtensionSelectOption {
  id: string;
  text: string;
}

export interface ExtensionShowCondition {
  setting_id: string;
  setting_value: string;
}

export interface ExtensionDefaultValues {
  extension_id: string;
  default_values: DefaultValue[];
}

export interface DefaultValue {
  setting_id: string;
  value: string;
}
