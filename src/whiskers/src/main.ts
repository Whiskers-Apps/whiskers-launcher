import { createApp } from "vue";
import "./styles.css";
import { createRouter, createWebHistory } from "vue-router";
import Root from "./Root.vue";
import Search from "@pages/Search/Search.vue";
import Settings from "@pages/Settings/SettingsPage.vue";
import AddToBlacklistDialog from "@pages/Settings/Dialogs/AddToBlacklist/AddToBlacklist.vue";
import ConfirmDeleteSearchEngineDialog from "@pages/Settings/Dialogs/ConfirmDeleteSearchEngine/ConfirmDeleteSearchEngine.vue";
import ConfirmDeleteExtensionDialog from "@pages/Settings/Dialogs/ConfirmUninstallExtension/ConfirmUninstallExtension.vue";
import AddSearchEngineDialog from "@pages/Settings/Dialogs/AddSearchEngine/AddSearchEngine.vue";
import EditSearchEngineDialog from "@pages/Settings/Dialogs/EditSearchEngine/EditSearchEngine.vue";
import CloneRepoDialog from "@pages/Settings/Dialogs/CloneRepo/CloneRepo.vue";
import ExtensionDialog from "@pages/Search/ExtensionDialog/ExtensionDialog.vue";
import ThemesStore from "@pages/Settings/Dialogs/ThemesStore/ThemesStore.vue";
import ExtensionsStore from "@pages/Settings/Dialogs/ExtensionsStore/ExtensionsStore.vue";

const routes = [
  { path: "/", component: Search },
  { path: "/settings", component: Settings },
  { path: "/add-to-blacklist", component: AddToBlacklistDialog },
  { path: "/confirm-delete-search-engine", component: ConfirmDeleteSearchEngineDialog },
  { path: "/add-search-engine", component: AddSearchEngineDialog },
  { path: "/edit-search-engine", component: EditSearchEngineDialog },
  { path: "/extension-dialog", component: ExtensionDialog },
  { path: "/clone-repo", component: CloneRepoDialog },
  { path: "/confirm-uninstall-extension", component: ConfirmDeleteExtensionDialog },
  { path: "/themes-store", component: ThemesStore },
  { path: "/extensions-store", component: ExtensionsStore },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const app = createApp(Root);
app.use(router);
app.mount("#app");
