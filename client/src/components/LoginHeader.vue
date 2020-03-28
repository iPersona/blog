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
        <!-- notifications -->
        <BNavbarItem>
          <div
            class="badge"
            :data-badge="notifyNum"
            @click="goToNotificationView"
          >
            <BellIcon
              size="1.5x"
              @click="goToNotificationView"
            />
          </div>
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
import { USER_NAME, TOKEN, NOTIFY_NUM } from '@/store/modules/store-types.js'
import { LOGOUT } from '@/store/modules/mutation-types.js'
import { USER } from '@/store/modules/module-names'
import Api from '@/api.js'
import { BellIcon, UserIcon, ChevronDownIcon, SettingsIcon, CreditCardIcon, LogOutIcon } from 'vue-feather-icons'

export default {
  name: 'LoginHeader',
  components: {
    BellIcon,
    UserIcon,
    ChevronDownIcon,
    SettingsIcon,
    CreditCardIcon,
    LogOutIcon,
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
      logout: LOGOUT,
    }),
    goToNotificationView() {
      this.$router.push({ name: 'notification' }).catch(err => { })
    }
  },
}
</script>

<style scoped>
.badge {
  display: inline-block;
  position: relative;
}
.badge[data-badge]:after {
  content: attr(data-badge);
  position: absolute;
  top: -0.1rem;
  right: -0.1rem;
  font-size: 0.7em;
  background: #ff3860;
  color: white;
  width: 15px;
  height: 15px;
  text-align: center;
  line-height: 15px;
  border-radius: 50%;
  box-shadow: 0 0 1px #333;
}

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