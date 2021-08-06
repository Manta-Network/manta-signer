import Vue from 'vue'
import VueRouter from 'vue-router'
import UnlockPage from '@/views/UnlockPage'
import RestoreVault from "@/views/RestoreVault";
import CreateAccount from "@/views/CreateAccount";

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'UnlockPage',
    component: UnlockPage
  },
  {
    path: '/create_account',
    name: 'CreateAccount',
    component: CreateAccount,
  },
  {
    path: '/restore_vault',
    name: 'RestoreVault',
    component: RestoreVault,
  }
]

const router = new VueRouter({
  routes
})

export default router
