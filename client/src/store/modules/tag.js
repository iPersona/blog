import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

import {
  SAVE_TAG,
} from './mutation-types'

import {
  TAG_ID,
  TAG_NAME
} from './store-types'


const state = {
  [TAG_ID]: undefined,
  [TAG_NAME]: undefined,
}

const mutations = {
  [SAVE_TAG](state, tag) {
    state[TAG_ID] = tag.id
    state[TAG_NAME] = tag.name
  },
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