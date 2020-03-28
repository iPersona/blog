import Vue from 'vue';
import App from './App.vue';

import axios from 'axios';
import VueAxios from 'vue-axios';
Vue.use(VueAxios, axios);

import router from './routes';

// buefy UI framework
import Buefy from 'buefy';
import 'buefy/dist/buefy.css';
Vue.use(Buefy)

// code highlight
import HighLight from './components/utils/highlight';
Vue.use(HighLight);

// localstorage
import VueLocalStorage from 'vue-localstorage';
Vue.use(VueLocalStorage);

import VueLogger from 'vuejs-logger';
const isProduction = process.env.NODE_ENV === 'production';
const options = {
  isEnabled: true,
  logLevel: isProduction ? 'error' : 'debug',
  stringifyArguments: false,
  showLogLevel: true,
  // showMethodName: true,
  separator: '|',
  showConsoleColors: true
};
Vue.use(VueLogger, options);

Vue.config.productionTip = false

// vuex
import store from '@/store/index.js'

// reCHAPTCHA-v3
import {
  VueReCaptcha
} from 'vue-recaptcha-v3'

// For more options see below
Vue.use(VueReCaptcha, {
  siteKey: '6LeU270UAAAAACZmn-pYBAgNt1hDTwaNmj8K8mVc',
  loaderOptions: {
    autoHideBadge: true,
  }
})

// ui control
import Ui from './components/utils/ui'
Vue.use(Ui)

// logger
import Log from './components/utils/log'
Vue.use(Log)

// scroll to element
import VueScrollTo from 'vue-scrollto'
Vue.use(VueScrollTo, {
  container: "body",
  duration: 500,
  easing: "ease",
  offset: -20,
  force: true,
  cancelable: true,
  onStart: false,
  onDone: false,
  onCancel: false,
  x: false,
  y: true
})

// clicpboard
import VueClipboard from 'vue-clipboard2'
Vue.use(VueClipboard)

// global css
import '@/assets/css/page.css'
import '@/assets/css/avatar.css'

new Vue({
  router,
  render: h => h(App),
  store,
}).$mount('#app')