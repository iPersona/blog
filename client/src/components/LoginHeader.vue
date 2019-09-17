<template>
  <div class="container">
    <b-navbar
      class="is-bold container"
      fixedTop
    >
      <template slot="brand">
        <b-navbar-item href="/">
          <img
            src="@/assets/logo.png"
            alt="Lightweight UI components for Vue.js based on Bulma"
          />
        </b-navbar-item>
      </template>

      <!-- Menu -->
      <!-- articles -->
      <template slot="start">
        <b-navbar-item
          tag="router-link"
          to="/articles"
        >
          <b>Articles</b>
        </b-navbar-item>

        <!-- tag -->
        <b-navbar-item
          tag="router-link"
          to="/tags"
        >
          <b>Tags</b>
        </b-navbar-item>
      </template>

      <template slot="end">
        <b-navbar-item>
          <b-dropdown
            v-model="item"
            position="is-bottom-left"
            aria-role="menu"
          >
            <button
              class="button rounded"
              type="button"
              slot="trigger"
            >
              <template>
                <b-icon icon="account"></b-icon>
                <span><b>{{userName}}</b></span>
                <b-icon icon="menu-down"></b-icon>
              </template>
            </button>
            <b-dropdown-item
              value="home"
              aria-role="menuitem"
            >
              <b-icon icon="home"></b-icon>
              Home
            </b-dropdown-item>
            <hr class="dropdown-divider" />
            <b-dropdown-item value="settings">
              <b-icon icon="settings"></b-icon>
              Settings
            </b-dropdown-item>
            <b-dropdown-item
              value="logout"
              aria-role="menuitem"
              @click="logout"
            >
              <b-icon icon="logout"></b-icon>
              Logout
            </b-dropdown-item>
          </b-dropdown>
        </b-navbar-item>
      </template>

    </b-navbar>
  </div>

</template>

<script>
import { mapGetters } from 'vuex'
import { mapMutations } from 'vuex'
import { USER_NAME, TOKEN } from '@/store-types.js'
import { LOGOUT } from '@/mutation-types.js'
import Api from '@/api.js'

export default {
  name: 'LoginHeader',
  components: {

  },
  data() {
    return {
      item: ''
    }
  },
  computed: {
    ...mapGetters({
      userName: [USER_NAME]
    }),
  },
  methods: {
    ...mapMutations({
      logout: LOGOUT,
    }),
  },
}
</script>