import { createApp } from "vue";
import { createRouter, createWebHistory } from 'vue-router'
import Index from './components/index.vue'
// import Coal from "./components/coal.vue";
import Progressbar from './components/Progressbar.vue'
import Fluent from './components/fluent.vue'
import test from "./components/test.vue";
import FracDirect from "./components/FracDirect.vue";
import table from "./components/table.vue";
import "./styles.css";
import App from "./App.vue";
import store from './store'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import  zhCn  from "element-plus/es/locale/lang/zh-cn";
const app = createApp(App);
app.component('Progressbar', Progressbar);
const router = createRouter({
    history: createWebHistory(),
    routes: [
      {path: '/',redirect: '/FracDirect'},
      { path: '/home/', name:'Home',component: ()=>import("./components/home.vue"),meta:{keepAlive:true} },
      { path: '/index/', name:'Index',component: Index,meta:{keepAlive:true} },
      { path: '/fluent/', name:'Fluent',component: Fluent,meta:{keepAlive:true} },
      { path: '/test', name:'Test',component: test,meta:{keepAlive:true} },
      {path: '/fracDirect/',name:'FracDirect',component: FracDirect,meta:{keepAlive:true}},
      {path: '/table/',name:'Table',component: table,meta:{keepAlive:true}}

    ],
})
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.use(ElementPlus, {locale: zhCn})
app.use(ElementPlus, { size: 'small', zIndex: 3000}).use(router).use(store).mount('#app')