import { createRouter, createWebHashHistory } from 'vue-router'
import Landing from '../components/Landing.vue'
import {defineAsyncComponent} from 'vue'

const routes = [
  {
    path: '/',
    name: 'Landing',
    component: Landing
  },
  {
    path: '/login',
    name: 'Login',
    component: defineAsyncComponent(() => import('../components/Login.vue'))
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: defineAsyncComponent(() => import('../components/Dashboard.vue'))
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router