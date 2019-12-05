import Vue from 'vue'
import Vuex from 'vuex'
import user from './modules/user'
import tag from './modules/tag'

Vue.use(Vuex)

const debug = process.env.NODE_ENV !== 'production'

const store = new Vuex.Store({
  modules: {
    user,
    tag
  },
  strict: debug,
})

export default store