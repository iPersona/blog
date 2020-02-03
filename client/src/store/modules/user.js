import Vue from 'vue'
import Vuex from 'vuex'
import jwt from 'jsonwebtoken'

Vue.use(Vuex)

export const STORE_KEY = 'coimioc-state'

import {
  LOGOUT,
  LOGIN,
  LOAD_USER,
  UPDATE_TOKEN,
} from './mutation-types'

import {
  USER_NAME,
  USER_ID,
  TOKEN,
  USER_INFO,
  IS_LOGIN,
  IS_ADMIN,
} from './store-types'
import {
  userInfo
} from 'os'

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
  state[USER_ID] = s[USER_ID]
  state[IS_LOGIN] = s[IS_LOGIN]
  state[TOKEN] = s[TOKEN]
  state[IS_ADMIN] = s[IS_ADMIN]
}

function updateToken(state, token) {
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
  state[USER_ID] = decoded.payload.user_id
  state[IS_ADMIN] = decoded.payload.is_admin
  saveState(state)
}

function userInfoFromPayload(payload) {
  return {
    name: payload.user_name,
    nickname: payload.user_nickname,
    email: payload.email,
    sign: payload.user_sign,
    registTime: payload.user_create_time,
  }
}

const state = {
  [USER_NAME]: '',
  [USER_ID]: '',
  [TOKEN]: '',
  [IS_LOGIN]: false,
  [IS_ADMIN]: false,
}

const mutations = {
  [LOGOUT](state) {
    state[IS_LOGIN] = false
    state[USER_NAME] = ''
    state[IS_ADMIN] = false
    clearState()
  },
  [LOGIN](state, token) {
    updateToken(state, token)
  },
  [LOAD_USER](state) {
    loadState(state)
  },
  [UPDATE_TOKEN](state, token) {
    updateToken(state, token)
  }
}

const getters = {
  [USER_NAME]: (state) => {
    return state[USER_NAME]
  },
  [USER_ID]: (state) => {
    return state[USER_ID]
  },
  [IS_LOGIN]: (state) => {
    return state[IS_LOGIN]
  },
  [IS_ADMIN]: (state) => {
    return state[IS_ADMIN]
  },
  [TOKEN]: (state) => {
    return state[TOKEN]
  },
  [USER_INFO]: (state) => {
    let token = state[TOKEN]
    // decode jwt
    let decoded = jwt.decode(token, {
      complete: true
    })

    if (decoded === null) {
      return
    }

    let ret = userInfoFromPayload(decoded.payload)
    console.log(`userinfo: ${JSON.stringify(ret)}`)
    return ret
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