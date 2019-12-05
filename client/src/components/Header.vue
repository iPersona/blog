<template>
  <!-- Visitor -->
  <div v-if="!isLogin">
    <VisitorHeader />
  </div>

  <!-- Admin -->
  <div v-else-if="isAdmin">
    <AdminHeader />
  </div>

  <!-- Register -->
  <div v-else>
    <LoginHeader />
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import LoginHeader from './LoginHeader'
import VisitorHeader from './VisitorHeader'
import AdminHeader from './AdminHeader'
import { USER_NAME, TOKEN, IS_LOGIN, IS_ADMIN } from '@/store/modules/store-types.js'
import { USER } from '@/store/modules/module-names'
// import { EventBus, EVENT_HIDE_HEADER, EVENT_SHOW_HEADER } from '@/event-bus.js'

export default {
  name: "Header",
  components: {
    LoginHeader,
    VisitorHeader,
    AdminHeader,
  },
  props: {},
  data() {
    return {
      // visible: true,
    };
  },
  computed: {
    ...mapGetters(USER, {
      userName: USER_NAME,
      isLogin: IS_LOGIN,
      isAdmin: IS_ADMIN,
    }),
  },
  mounted() {
    // await this.listenEventBus()
  },
  methods: {
    // listenEventBus() {
    //   const self = this;
    //   EventBus.$on(EVENT_HIDE_HEADER, async function () {
    //     self.visible = false
    //   })
    //   EventBus.$on(EVENT_SHOW_HEADER, async function () {
    //     self.visible = true
    //   })
    // },
  }
};
</script>