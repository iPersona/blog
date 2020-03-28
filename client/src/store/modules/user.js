import Vue from 'vue'
import Vuex from 'vuex'
import jwt from 'jsonwebtoken'

Vue.use(Vuex)

export const STORE_KEY = 'coimioc-state'

import {
  LOGOUT,
  LOGIN,
  LOAD_USER,
  UPDATE_LOGIN_DATA,
  DECREASE_NOTIFICATION_NUM
} from './mutation-types'

import {
  USER_NAME,
  USER_ID,
  TOKEN,
  USER_INFO,
  IS_LOGIN,
  IS_ADMIN,
  NOTIFY_NUM,
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
  console.log('aaaaa')
  let s = getState()
  if (s === null) {
    return
  }

  state[USER_NAME] = s[USER_NAME]
  state[USER_ID] = s[USER_ID]
  state[IS_LOGIN] = s[IS_LOGIN]
  state[TOKEN] = s[TOKEN]
  state[IS_ADMIN] = s[IS_ADMIN]
  state[NOTIFY_NUM] = s[NOTIFY_NUM]
}

function updateLoginData(state, data) {
  if (data.notify_num === undefined) {
    state[NOTIFY_NUM] = 0
  } else {
    state[NOTIFY_NUM] = data.notify_num
  }

  if (data.token === undefined || data.token === null) {
    return
  }

  // decode jwt
  let decoded = jwt.decode(data.token, {
    complete: true
  })

  if (decoded === null) {
    return
  }

  state[IS_LOGIN] = true
  state[TOKEN] = data.token
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

function decreaseNotificationNum(state) {
  state[NOTIFY_NUM] -= 1
  saveState(state)
}

const state = {
  [USER_NAME]: '',
  [USER_ID]: '',
  [TOKEN]: '',
  [IS_LOGIN]: false,
  [IS_ADMIN]: false,
  [NOTIFY_NUM]: 0,
}

const mutations = {
  [LOGOUT](state) {
    state[IS_LOGIN] = false
    state[USER_NAME] = ''
    state[IS_ADMIN] = false
    state[NOTIFY_NUM] = 0
    clearState()
    // refresh page to reset component
    window.location.reload()
  },
  [LOGIN](state, data) {
    updateLoginData(state, data)
  },
  [LOAD_USER](state) {
    loadState(state)
  },
  [UPDATE_LOGIN_DATA](state, data) {
    updateLoginData(state, data)
  },
  [DECREASE_NOTIFICATION_NUM](state) {
    decreaseNotificationNum(state)
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
  [NOTIFY_NUM]: (state) => {
    return state[NOTIFY_NUM]
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