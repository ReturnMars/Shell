import { createApp } from "vue";
import { router } from "./router";
import App from "./App.vue";
// 通用字体
import "vfonts/Lato.css";
// 等宽字体
import "vfonts/FiraCode.css";
// 全局样式
import "./assets/styles/index.scss";
import "virtual:uno.css";

const app = createApp(App);
app.use(router);
app.mount("#app");
