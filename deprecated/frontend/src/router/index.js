import Vue from 'vue';
import VueRouter from 'vue-router';
import AuthorizeTransaction from '@/views/AuthorizeTransaction';
import Home from '@/views/Home';
import SignIn from '@/views/SignIn';
import CreateAccount from '@/views/CreateAccount';
import CreateNewAccount from '@/views/CreateNewAccount';
import RecoverAccount from '@/views/RecoverAccount';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/authorize_transaction/:txType/:amount/:currencySymbol/:recipient',
    name: 'AuthorizeTransaction',
    component: AuthorizeTransaction,
  },
  {
    path: '/create_account',
    name: 'CreateAccount',
    component: CreateAccount,
  },
  {
    path: '/create_new_account',
    name: 'CreateNewAccount',
    component: CreateNewAccount,
  },
  {
    path: '/recover_account',
    name: 'RecoverAccount',
    component: RecoverAccount,
  },
  {
    path: '/sign_in',
    name: 'SignIn',
    component: SignIn,
  },
];

const router = new VueRouter({
  routes,
});

export default router;
