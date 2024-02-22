import { invoke } from "@tauri-apps/api";
import { Settings } from "@pages/Settings/ViewModel";
import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
import { getIconUrl, getScaledSize } from "@/utils";
import { ref, watch } from "vue";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

export interface UiState {
  typedText: string;
}

export class ViewModel {
  hasLoaded = false;
  settings: Settings | null = null;

  typedText = ref("");
  results: WhiskersResult[] = [];
  resultHeight = 0;
  resultsHeight = "0px";

  selectedIndex = 0;

  async load() {
    this.settings = await invoke("get_settings");
    this.resultHeight = this.getResultHeight();
  }

  getSearchInputBackground(): string {
    let background = this.settings!!.theme.background_main;

    if (["highlight", "highlight_split"].includes(this.settings!!.layout)) {
      background = this.settings!!.theme.background_secondary;
    }

    return background;
  }

  getSearchDivClasses(): string {
    return this.isSplitLayout() ? "search-box " : "";
  }

  isSplitLayout(): boolean {
    return ["highlight_split", "modern_split"].includes(this.settings!!.layout);
  }

  isHighlightLayoutType(): boolean {
    return ["highlight", "highlight_split"].includes(this.settings!!.layout);
  }

  isModernLayoutType(): boolean {
    return ["modern", "modern_split"].includes(this.settings!!.layout);
  }

  getInputPlaceholder(): string {
    return this.settings!!.show_placeholder ? "Type to search" : "";
  }

  getAppIconSrc(icon: string | null): string {
    let src = "";

    if (icon !== null) {
      src = convertFileSrc(icon);
    } else {
      src = getIconUrl("default-app-icon.svg");
    }

    return src;
  }

  async search() {
    this.selectedIndex = 0;
    document.getElementById("results-div")?.scrollTo({ top: 0 });
    this.results = await invoke("get_search_results", {
      typed_text: this.typedText,
    });

    let totalHeight =
      (this.results.length < this.settings!!.results_count
        ? this.results.length
        : this.settings!!.results_count) * this.getResultHeight();

    this.resultsHeight = `${totalHeight}px`;
  }

  onArrowDownPress() {
    if (this.selectedIndex < this.results.length - 1) {
      this.selectedIndex += 1;

      document
        .getElementById(`result-${this.selectedIndex - 1}`)!!
        .scrollIntoView({ behavior: "smooth" });
    } else if (this.selectedIndex == this.results.length - 1) {
      this.selectedIndex = 0;

      document.getElementById(`result-0`)!!.scrollIntoView({ behavior: "smooth" });
    }
  }

  onArrowUpPress() {
    if (this.selectedIndex > 0) {
      this.selectedIndex -= 1;

      document
        .getElementById(`result-${this.selectedIndex - 1}`)!!
        .scrollIntoView({ behavior: "smooth" });
    } else if (this.selectedIndex == 0) {
      this.selectedIndex = this.results.length - 1;

      document
        .getElementById(`result-${this.selectedIndex - 1}`)!!
        .scrollIntoView({ behavior: "smooth" });
    }
  }

  async runAction() {
    const result = this.results[this.selectedIndex];
    const action = result.action!!;

    if (action.type === "OpenApp") {
      invoke("open_app", { exec_path: action.path ?? "" });
    }

    if (action.type === "OpenUrl") {
      invoke("open_url", { url: action.url ?? "" });
    }

    if (action.type === "CopyToClipboard") {
      await writeText(action.text!!);
    }

    if (action.type === "Extension") {
      invoke("run_extension_action", {
        extension_id: action.extension_id,
        extension_action: action.extension_action,
        args: action.args,
      });
    }

    if (action.type === "Dialog") {
      await invoke("open_extension_dialog", {
        extension_id: action.extension_id,
        extension_action: action.extension_action,
        title: action.title,
        primary_button_text: action.primary_button_text,
        fields: action.fields,
        args: action.args,
      });

      new WebviewWindow("extension-dialog", {
        url: "extension-dialog",
        title: action.title!!,
        width: 700,
        height: 700,
        center: true
      });

      setTimeout(() => {
        appWindow.close();
      }, 500);
    }
  }

  getResultHeight(): number {
    let overallPadding = 0;
    let appIconSize = 0;
    const settings = this.settings!!;
    const fractionalScaling = settings.fractional_scaling;

    if (settings.density === "small") {
      overallPadding = getScaledSize(fractionalScaling, 6);
      appIconSize = getScaledSize(fractionalScaling, 32);
    } else if (settings.density === "large") {
      overallPadding = getScaledSize(fractionalScaling, 10);
      appIconSize = getScaledSize(fractionalScaling, 48);
    } else {
      overallPadding = getScaledSize(fractionalScaling, 8);
      appIconSize = getScaledSize(fractionalScaling, 40);
    }

    return appIconSize + overallPadding + overallPadding + overallPadding;
  }

  openSettings(event: Event | undefined = undefined) {
    event?.stopPropagation();

    new WebviewWindow("settings", {
      url: "settings",
      title: "Settings",
      width: 1100,
      height: 700,
    });

    setTimeout(() => {
      appWindow.close();
    }, 500);
  }
}

export interface WhiskersResult {
  type: string;
  icon: string | null;
  tint_icon: boolean | null;
  tint_color: string | null;
  title: string | null;
  text: string | null;
  action: WhiskersAction | null;
}

export interface WhiskersAction {
  type: string;
  path: string | null;
  url: string | null;
  title: string | null;
  primary_button_text: string | null;
  text: string | null;
  extension_id: string | null;
  extension_action: string | null;
  args: string[] | null;
  fields: DialogField[];
}

export interface DialogField {
  type: string;
  id: string;
  title: string;
  value: string | null;
  description: string | null;
  placeholder: string | null;
  custom_args: string | null;
  toggled: boolean | null;
  default_field_id: string | null;
  fields: DialogSelectField[] | null;
  select_dir: boolean | null;
  default_path: string | null;
  filters: FileFilter[] | null;
}

export interface DialogSelectField {
  id: string;
  text: string;
}

export interface FileFilter {
  name: string;
  file_extensions: string[];
}
