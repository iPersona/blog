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
          <plus-circle-icon
            size="1.5x"
            @click="newPost"
          />
        </BNavbarItem>
        <BNavbarItem>
          <BellIcon
            size="1.5x"
            @click="goToNotificationView"
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
                <UserIcon size="1.5x" />
                <span class="dropdown-btn-text"><b>{{ userName }}</b></span>
                <ChevronDownIcon size="1.5x" />
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
                <template>
                  <CreditCardIcon
                    size="1.5x"
                    class="menu_icon"
                  />
                  <span class="menu_text">
                    Profile
                  </span>
                </template>
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
                <template>
                  <SettingsIcon
                    size="1.5x"
                    class="menu_icon"
                  />
                  <span class="menu_text">
                    Settings
                  </span>
                </template>
              </router-link>
            </BDropdownItem>

            <BDropdownItem
              value="logout"
              aria-role="menuitem"
              @click="logout"
            >
              <template>
                <LogOutIcon
                  size="1.5x"
                  class="menu_icon"
                />
                <span class="menu_text">
                  Logout
                </span>
              </template>
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
import { PlusCircleIcon, BellIcon, UserIcon, ChevronDownIcon, SettingsIcon, CreditCardIcon, LogOutIcon } from 'vue-feather-icons';

export default {
  name: 'AdminHeader',
  components: {
    ArticleEditor,
    PlusCircleIcon,
    BellIcon,
    UserIcon,
    ChevronDownIcon,
    SettingsIcon,
    CreditCardIcon,
    LogOutIcon,
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
    goToNotificationView() {
      this.$router.push({ name: 'notifications' })
    }
  },
}
</script>

<style scoped>
.dropdown-btn-text {
  margin-left: 0.3rem;
  margin-right: 0.3rem;
}

.menu_icon {
  display: inline-block;
  vertical-align: middle;
}

.menu_text {
  margin-left: 0.5rem;
}
</style>