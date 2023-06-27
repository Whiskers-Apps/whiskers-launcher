import { createApp } from "vue";
import "./styles.css";
import Root from "./Root.vue"
import App from "./App.vue";
import Settings from "./pages/Settings/Settings.vue"
import { createRouter, createWebHistory } from "vue-router"


const routes = [
    { path: "/", name: "app", component: App },
    { path: "/settings", name: "settings", component: Settings }
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

const app = createApp(Root);
app.use(router);
app.mount("#app");
