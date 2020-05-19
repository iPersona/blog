import Vue from 'vue'
import Vuex from 'vuex'
import jwt from 'jsonwebtoken'

Vue.use(Vuex)

import {
  LOGOUT,
  CLEAR_STATE,
  LOGIN,
  UPDATE_LOGIN_DATA,
  DECREASE_NOTIFICATION_NUM
} from './mutation-types'

import {
  USER_NAME,
  USER_ID,
  USER_INFO,
  IS_LOGIN,
  IS_ADMIN,
  NOTIFY_NUM,
} from './store-types'

function updateLoginData(state, data) {
  if (data.notify_num === undefined) {
    state[NOTIFY_NUM] = 0
  } else {
    state[NOTIFY_NUM] = data.notify_num
  }

  state[IS_LOGIN] = data.is_login
  state[USER_NAME] = data.user_name
  state[USER_ID] = data.user_id
  state[IS_ADMIN] = data.is_admin
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
}

const state = {
  [USER_NAME]: '',
  [USER_ID]: '',
  [IS_LOGIN]: false,
  [IS_ADMIN]: false,
  [NOTIFY_NUM]: 0,
}

function clearState(state) {
  state[USER_NAME] = ''
  state[USER_ID] = ''
  state[IS_LOGIN] = false
  state[IS_ADMIN] = false
  state[NOTIFY_NUM] = 0
}

const mutations = {
  [CLEAR_STATE](state) {
    clearState(state)
  },
  [LOGOUT](state) {
    clearState(state)
    // refresh page to reset component
    window.location.reload()
  },
  [LOGIN](state, data) {
    updateLoginData(state, data)
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