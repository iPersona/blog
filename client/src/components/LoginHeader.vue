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
      </template>

      <template slot="end">
        <!-- notifications -->
        <BNavbarItem>
          <BadgeIcon
            :number="notifyNum"
            icon="bell"
            @clickEvent="goToNotificationView"
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
            >
              <CreditCardIcon
                class="menu_icon"
                size="1.5x"
              />
              <span class="menu_text">
                Profile
              </span>
            </BDropdownItem>
            <hr class="dropdown-divider">
            <BDropdownItem value="settings">
              <SettingsIcon
                class="menu_icon"
                size="1.5x"
              />
              <span class="menu_text">
                Settings
              </span>
            </BDropdownItem>
            <BDropdownItem
              value="logout"
              aria-role="menuitem"
              @click="logout"
            >
              <LogOutIcon
                class="menu_icon"
                size="1.5x"
              />
              <span class="menu_text">
                Logout
              </span>
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
import { USER_NAME, NOTIFY_NUM } from '@/store/modules/store-types.js'
import { LOGOUT } from '@/store/modules/mutation-types.js'
import { USER } from '@/store/modules/module-names'
import Api from '@/api.js'
import { BellIcon, UserIcon, ChevronDownIcon, SettingsIcon, CreditCardIcon, LogOutIcon } from 'vue-feather-icons'
import BadgeIcon from './controllers/BadgeIcon'

export default {
  name: 'LoginHeader',
  components: {
    UserIcon,
    ChevronDownIcon,
    SettingsIcon,
    CreditCardIcon,
    LogOutIcon,
    BadgeIcon,
  },
  data() {
    return {
      item: ''
    }
  },
  computed: {
    ...mapGetters(USER, {
      userName: [USER_NAME],
      notifyNum: [NOTIFY_NUM]
    }),
  },
  mounted() {
  },
  methods: {
    ...mapMutations(USER, {
      clearSession: LOGOUT,
    }),
    goToNotificationView() {
      this.$router.push({ name: 'notification' }).catch(err => { })
    },
    async logout() {
      let api = new Api()
      let rsp = await api.logout()
      if (!rsp.isSuccess()) {
        this.$getUi().toast.fail(`failed to logout: ${rsp.errorDetail()}`)
        return
      }
      this.clearSession()
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