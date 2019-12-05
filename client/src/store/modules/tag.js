import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

import {
  SAVE_TAG,
  LOAD_TAG
} from './mutation-types'

import {
  TAG_ID,
  TAG_NAME
} from './store-types'

const STORE_KEY = 'tag-store'

function saveState(s) {
  sessionStorage.setItem(STORE_KEY, s)
}

function clearState() {
  sessionStorage.removeItem(STORE_KEY)
}

function getState() {
  return sessionStorage.getItem(STORE_KEY)
}

function loadState(state) {
  let s = getState()
  if (s === null) {
    return
  }

  state[TAG_ID] = s[TAG_ID]
  state[TAG_NAME] = s[TAG_NAME]
}

const state = {
  [TAG_ID]: undefined,
  [TAG_NAME]: undefined,
}

const mutations = {
  [SAVE_TAG](state, tag) {
    console.log(`update state 99999: ${tag.id}, ${tag.name}`)
    state[TAG_ID] = tag.id
    state[TAG_NAME] = tag.name
    saveState(state) // save state to session storage
  },
  [LOAD_TAG](state) {
    loadState(state)
  }
}

const getters = {
  [TAG_ID]: (state) => {
    console.log(`TAG_ID: ${state[TAG_ID]}`)
    return state[TAG_ID]
  },
  [TAG_NAME]: (state) => {
    console.log(`TAG_NAME: ${state[TAG_NAME]}`)
    return state[TAG_NAME]
  },
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