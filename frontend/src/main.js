import Vue from 'vue'
import App from './App.vue'
import {ready} from '@wails/runtime'
import './plugins/element.js'
import router from './router'

Vue.config.productionTip = false

ready(() => {
  new Vue({
    router,
    render: h => h(App)
  }).$mount('#app')
})
