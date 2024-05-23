import { createApp } from "vue";
import { createRouter, createWebHistory } from 'vue-router'
import Index from './components/index.vue'
import Coal from "./components/coal.vue";
import Progressbar from './components/Progressbar.vue'
import Fluent from './components/fluent.vue'
import test from "./components/test.vue";
import "./styles.css";
import App from "./App.vue";
import store from './store'
const app = createApp(App);
app.component('Progressbar', Progressbar);
const router = createRouter({
    history: createWebHistory(),
    routes: [
      {path: '/',redirect: '/coal'},
      { path: '/home/', name:'Home',component: ()=>import("./components/home.vue"),meta:{keepAlive:true} },
      { path: '/index/', name:'Index',component: Index,meta:{keepAlive:true} },
      { path: '/fluent/', name:'Fluent',component: Fluent,meta:{keepAlive:true} },
      { 
        path: '/coal/', 
        name:'Coal',
        component: Coal,
        meta:{keepAlive:true},
      },
      { path: '/test', name:'Test',component: test,meta:{keepAlive:true} },
    ],
})

app.use(router).use(store).mount('#app')