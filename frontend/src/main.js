import Vue from 'vue';
import App from './App.vue';
import { ready, Events } from '@wails/runtime';
import './plugins/element.js';
import router from './router';
import './assets/main.css';

Vue.config.productionTip = false;

ready(() => {
  new Vue({
    router,
    render: h => h(App),
  }).$mount('#app');
});

// After Ready
Events.On('manta.browser.openCreate', function() {
  // check the path match
  if (router.currentRoute.path !== '/create_account') {
    router.push('/create_account');
  }
});
Events.On('manta.browser.openUnlock', function() {
  if (router.currentRoute.path !== '/unlock') {
    router.push('/unlock');
  }
});
