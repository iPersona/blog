import Vue from 'vue'
import Vuex from 'vuex'
import jwt from 'jsonwebtoken'

Vue.use(Vuex)

export const STORE_KEY = 'coimioc-state'

import {
  LOGOUT,
  LOGIN,
  LOAD_USER
} from './mutation-types'

import {
  USER_NAME,
  TOKEN,
  IS_LOGIN,
  IS_ADMIN,
} from './store-types'

function saveState(s) {
  console.log(`saveState...`)
  let data = btoa(JSON.stringify(s))
  localStorage.setItem(STORE_KEY, data)
}

function clearState() {
  localStorage.removeItem(STORE_KEY)
}

function getState() {
  let encoded = localStorage.getItem(STORE_KEY)
  if (encoded === null) {
    return null
  }
  let decoded = atob(encoded)
  return JSON.parse(decoded)
}

function loadState(state) {
  let s = getState()
  if (s === null) {
    return
  }

  state[USER_NAME] = s[USER_NAME]
  state[IS_LOGIN] = s[IS_LOGIN]
  state[TOKEN] = s[TOKEN]
  state[IS_ADMIN] = s[IS_ADMIN]
}

const state = {
  [USER_NAME]: '',
  [TOKEN]: '',
  [IS_LOGIN]: false,
  [IS_ADMIN]: false,
}

const mutations = {
  // 我们可以使用 ES2015 风格的计算属性命名功能来使用一个常量作为函数名
  [LOGOUT](state) {
    state[IS_LOGIN] = false
    state[USER_NAME] = ''
    state[IS_ADMIN] = false
    clearState()
  },
  [LOGIN](state, token) {
    if (token === undefined || token === null) {
      return
    }

    // decode jwt
    let decoded = jwt.decode(token, {
      complete: true
    })

    if (decoded === null) {
      return
    }

    state[IS_LOGIN] = true
    state[TOKEN] = token
    state[USER_NAME] = decoded.payload.user_name
    state[IS_ADMIN] = decoded.payload.is_admin
    saveState(state)
  },
  [LOAD_USER](state) {
    loadState(state)
  }
}

const getters = {
  [USER_NAME]: (state) => {
    // loadState(state)
    return state[USER_NAME]
  },
  [IS_LOGIN]: (state) => {
    // loadState(state)
    return state[IS_LOGIN]
  },
  [IS_ADMIN]: (state) => {
    // loadState(state)
    return state[IS_ADMIN]
  },
  [TOKEN]: (state) => {
    // loadState(state)
    return state[TOKEN]
  }
}

const actions = {}

const store = {
  namespaced: true,
  state,
  mutations,
  getters,
  actions,
}

export default store