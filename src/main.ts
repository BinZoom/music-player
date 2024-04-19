import { createApp } from "vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import "./styles.css";
import App from "./App.vue";

createApp(App)
.use(ElementPlus) // 启用 ElementPlus
.mount("#app");
