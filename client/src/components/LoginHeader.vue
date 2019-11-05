<template>
  <div class="container">
    <BNavbar
      class="is-bold container"
      fixed-top
      shadow
    >
      <template slot="brand">
        <BNavbarItem href="/">
          <img
            src="@/assets/logo.png"
            alt="Lightweight UI components for Vue.js based on Bulma"
          >
        </BNavbarItem>
      </template>

      <!-- Menu -->
      <!-- articles -->
      <template slot="start">
        <BNavbarItem
          tag="router-link"
          to="/articles"
        >
          <b>Articles</b>
        </BNavbarItem>

        <!-- tag -->
        <BNavbarItem
          tag="router-link"
          to="/tags"
        >
          <b>Tags</b>
        </BNavbarItem>
      </template>

      <template slot="end">
        <BNavbarItem>
          <BDropdown
            v-model="item"
            position="is-bottom-left"
            aria-role="menu"
          >
            <button
              slot="trigger"
              class="button rounded"
              type="button"
            >
              <template>
                <BIcon icon="account" />
                <span><b>{{ userName }}</b></span>
                <BIcon icon="menu-down" />
              </template>
            </button>
            <BDropdownItem
              value="home"
              aria-role="menuitem"
            >
              <BIcon icon="home" />
              Home
            </BDropdownItem>
            <hr class="dropdown-divider">
            <BDropdownItem value="settings">
              <BIcon icon="settings" />
              Settings
            </BDropdownItem>
            <BDropdownItem
              value="logout"
              aria-role="menuitem"
              @click="logout"
            >
              <BIcon icon="logout" />
              Logout
            </BDropdownItem>
          </BDropdown>
        </BNavbarItem>
      </template>
    </BNavbar>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import { mapMutations } from 'vuex'
import { USER_NAME, TOKEN } from '@/store/modules/store-types.js'
import { LOGOUT } from '@/store/modules/mutation-types.js'
import { USER } from '@/store/modules/module-names'
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
    ...mapGetters(USER, {
      userName: [USER_NAME]
    }),
  },
  methods: {
    ...mapMutations(USER, {
      logout: LOGOUT,
    }),
  },
}
</script>