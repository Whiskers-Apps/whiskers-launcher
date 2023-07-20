import { createApp } from "vue";
import "./styles.css";
import Root from "./Root.vue"
import Search from "./pages/Search/Search.vue"
import Settings from "./pages/Settings/Settings.vue"
import EditSearchEngineDialogVue from "./pages/Dialogs/EditSearchEngineDialog.vue";
import { createRouter, createWebHistory } from "vue-router"
import DeleteSearchEngineDialogVue from "./pages/Dialogs/DeleteSearchEngineDialog.vue";


const routes = [
    { path: "/", name: "search", component: Search },
    { path: "/settings", name: "settings", component: Settings },
    { path: "/edit-search-engine/:index", name: "edit-search-engine", component: EditSearchEngineDialogVue },
    { path: "/delete-search-engine", name: "delete-search-engine", component: DeleteSearchEngineDialogVue }
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

const app = createApp(Root);
app.use(router);
app.mount("#app");
