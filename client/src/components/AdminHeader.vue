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
            alt="Welcome home"
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

        <!-- Manage -->
        <BNavbarItem
          tag="router-link"
          to="/management"
        >
          <b>Management</b>
        </BNavbarItem>
      </template>

      <template slot="end">
        <BNavbarItem>
          <BIcon
            pack="fas"
            icon="plus-circle"
            size="is-medium"
            @click.native="newPost"
          />
        </BNavbarItem>
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
              value="profile"
              aria-role="menuitem"
              has-link
            >
              <router-link
                v-model="profile"
                :to="{name: 'profile'}"
                exact
              >
                <BIcon
                  pack="fas"
                  icon="id-card"
                />
                <span>
                  Profile
                </span>
              </router-link>
            </BDropdownItem>

            <hr class="dropdown-divider">

            <BDropdownItem
              value="settings"
              aria-role="menuitem"
              has-link
            >
              <router-link
                v-model="settings"
                :to="{name: 'settings'}"
                exact
              >
                <BIcon
                  pack="fas"
                  icon="cog"
                />
                <span>
                  Settings
                </span>
              </router-link>
            </BDropdownItem>

            <BDropdownItem
              value="logout"
              aria-role="menuitem"
              @click="logout"
            >
              <BIcon
                pack="fas"
                icon="sign-out-alt"
              />
              <span>Logout</span>
            </BDropdownItem>
          </BDropdown>
        </BNavbarItem>
      </template>
    </BNavbar>

    <BModal
      :active.sync="isEditArticle"
      has-modal-card
      full-screen
      :can-cancel="false"
    >
      <ArticleEditor :is-create-new="true" />
    </BModal>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import { mapMutations } from 'vuex'
import { USER_NAME, TOKEN } from '@/store/modules/store-types.js'
import { LOGOUT } from '@/store/modules/mutation-types.js'
import { USER } from '@/store/modules/module-names'
import Api from '@/api.js'
import ArticleEditor from './ArticleEditor'

export default {
  name: 'AdminHeader',
  components: {
    ArticleEditor,
  },
  data() {
    return {
      item: '',
      isEditArticle: false,

      // the props below is defined just for ignoring vue warning:
      // Property or method is not defined on the instance but referenced during render.
      profile: 'profile',
      settings: 'settings'
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
    newPost() {
      // this.$router.push({ name: 'new_post' })
      this.isEditArticle = true
    },
  },
}
</script>