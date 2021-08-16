import Vue from 'vue'
import App from './App.vue'
import {ready, Events} from '@wails/runtime'
import './plugins/element.js'
import router from './router'

Vue.config.productionTip = false

ready(() => {
  new Vue({
    router,
    render: h => h(App)
  }).$mount('#app')
})

// After Ready
Events.On('manta.browser.openCreate', function() {
  // 判断是否当前的path是否一致
  if (router.currentRoute.path !== '/create_account') {
    router.push('/create_account')
  }
})
Events.On("manta.browser.openUnlock", function () {
  if (router.currentRoute.path !== '/unlock') {
    router.push('/unlock')
  }
})