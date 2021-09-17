import Vue from 'vue';
import VueRouter from 'vue-router';
import UnlockPage from '@/views/UnlockPage';
import SignInPage from '@/views/SignInPage';
import CreateAccount from '@/views/CreateAccount';

Vue.use(VueRouter);

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
    path: '/sign_in',
    name: 'SignIn',
    component: SignInPage,
  },
];

const router = new VueRouter({
  routes,
});

export default router;
