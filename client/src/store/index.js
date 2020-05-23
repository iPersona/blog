import Vuex from 'vuex'

import user from './modules/user'
import tag from './modules/tag'

import createPersistedState from "vuex-persistedstate"
import SecureLS from "secure-ls"

const ls = new SecureLS({
  encodingType: 'aes',
  isCompression: false
})

const store = new Vuex.Store({
  modules: {
    user,
    tag
  },
  plugins: [
    createPersistedState({
      paths: [
        'user'
      ],
      storage: {
        getItem: key => ls.get(key),
        setItem: (key, value) => ls.set(key, value),
        removeItem: key => ls.remove(key)
      }
    }),
    createPersistedState({
      paths: [
        'tag'
      ],
      storage: window.sessionStorage
    }),
  ]
})

export default store