import Vue from 'vue'
import VueRouter from 'vue-router'
import UnlockPage from '@/views/UnlockPage'
import RestoreVault from "@/views/RestoreVault";
import CreateAccount from "@/views/CreateAccount";
import backend from "@/backend";

Vue.use(VueRouter)

const routes = [
  {
    path: '/unlock',
    name: 'UnlockPage',
    component: UnlockPage,
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
  },
]

const router = new VueRouter({
  routes
})

router.beforeEach((to, from, next) => {
  if (to.path !== '/create_account') {
    backend.main.Service.AccountCreated().then((created) => {
      if (!created) {
        next({
          path: '/create_account'
        })
      }
    })
  }
  next();
})

export default router
