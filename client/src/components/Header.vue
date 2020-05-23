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
import { USER_NAME, IS_LOGIN, IS_ADMIN } from '@/store/modules/store-types.js'
import { USER } from '@/store/modules/module-names'

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
  },
  methods: {
  }
};
</script>