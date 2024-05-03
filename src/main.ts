import { createApp } from "vue";
import { createRouter, createWebHistory } from 'vue-router'
import Home from './components/home.vue'
import Index from './components/index.vue'
import Coal from "./components/coal.vue";
import Progressbar from './components/Progressbar.vue'
import Fluent from './components/fluent.vue'
import "./styles.css";
import App from "./App.vue";
const app = createApp(App);
app.component('Progressbar', Progressbar);
const router = createRouter({
    history: createWebHistory(),
    routes: [
      {path: '/',redirect: '/Home'},
      { path: '/home', name:'Home',component: Home,meta:{keepAlive:true} },
      { path: '/index', name:'Index',component: Index,meta:{keepAlive:true} },
      { path: '/fluent', name:'Fluent',component: Fluent,meta:{keepAlive:true} },
      { path: '/coal', name:'Coal',component: Coal,meta:{keepAlive:true} },
    ],
})

createApp(App).use(router).mount('#app')