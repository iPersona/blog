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

// permission control
// import abilitiesPlugin from '@casl/vue';
// import ability from './components/utils/ability';
// Vue.use(abilitiesPlugin, ability);

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
  showMethodName: true,
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

new Vue({
  router,
  render: h => h(App),
  store,
}).$mount('#app')