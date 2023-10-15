import { createApp } from "vue";
import "./styles.css";
import { createRouter, createWebHistory } from "vue-router"
import Root from "./Root.vue"
import Search from "@pages/Search/Search.vue"
import Settings from "@pages/Settings/Settings.vue"
import EditSearchEngineDialog from "@dialogs/EditSearchEngineDialog.vue";
import DeleteSearchEngineDialog from "@dialogs/DeleteSearchEngineDialog.vue";
import AddSearchEngineDialog from "@dialogs/AddSearchEngineDialog.vue";
import ImportExtensionDialog from "@dialogs/ImportExtensionDialog.vue"
import DeleteExtensionDialog from "@dialogs/DeleteExtensionDialog.vue"
import CommunityThemesDialog from "@dialogs/CommunityThemesDialog.vue"
import CommunityExtensionsDialog from "@dialogs/CommunityExtensionsDialog.vue"
import ExtensionDialog from "@dialogs/ExtensionDialog.vue"
import AddToBlacklistDialogVue from "@dialogs/AddToBlacklistDialog.vue";

const routes = [
    { path: "/", name: "search", component: Search },
    { path: "/settings", name: "settings", component: Settings },
    { path: "/edit-search-engine/:index", name: "edit-search-engine", component: EditSearchEngineDialog },
    { path: "/delete-search-engine", name: "delete-search-engine", component: DeleteSearchEngineDialog },
    { path: "/add-search-engine", name: "add-search-engine", component: AddSearchEngineDialog },
    { path: "/import-extension-dialog", name: "import-extension-dialog", component: ImportExtensionDialog },
    { path: "/delete-extension-dialog", name: "delete-extension-dialog", component: DeleteExtensionDialog },
    { path: "/community-themes-dialog", name: "community-themes-dialog", component: CommunityThemesDialog },
    { path: "/community-extensions-dialog", name: "community-extensions-dialog", component: CommunityExtensionsDialog },
    { path: "/extension-dialog", name: "extension-dialog", component: ExtensionDialog },
    { path: "/add-to-blacklist-dialog", name: "add-to-blacklist-dialog", component: AddToBlacklistDialogVue }
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

const app = createApp(Root);
app.use(router);
app.mount("#app");
