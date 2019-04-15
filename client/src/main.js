import Vue from 'vue'
import App from './App.vue'

import BootstrapVue from 'bootstrap-vue'
Vue.use(BootstrapVue)

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap/scss/bootstrap.scss'

import axios from 'axios'
import VueAxios from 'vue-axios'
Vue.use(VueAxios, axios)

import router from './routes'

import TreeView from "vue-json-tree-view"
Vue.use(TreeView)

import LiquorTree from 'liquor-tree'
Vue.use(LiquorTree)

import ElementUI from 'element-ui'
import 'element-ui/lib/theme-chalk/index.css'
Vue.use(ElementUI)

require('vue-ionicons/ionicons.css')

Vue.config.productionTip = false

new Vue({
  router,
  render: h => h(App),
}).$mount('#app')