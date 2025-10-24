import { createApp } from "vue";
import { createPinia } from "pinia";
import "normalize.css";
import "./style.css";
import App from "./App.vue";
import "vfonts/Lato.css";
import "vfonts/FiraCode.css";
import "virtual:uno.css";
const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.mount("#app");
