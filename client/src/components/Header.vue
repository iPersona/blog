<template>
  <!-- Visitor -->
  <div v-if="!isLogin">
    <visitor-header />
  </div>

  <!-- Admin -->
  <div v-else-if="isAdmin">
    <admin-header />
  </div>

  <!-- Register -->
  <div v-else>
    <login-header />
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import LoginForm from './Login'
import SignupForm from './Signup'
import LoginHeader from './LoginHeader'
import VisitorHeader from './VisitorHeader'
import AdminHeader from './AdminHeader'
import { USER_NAME, TOKEN, IS_LOGIN, IS_ADMIN } from '@/store-types.js'

export default {
  name: "Header",
  components: {
    LoginForm,
    SignupForm,
    LoginHeader,
    VisitorHeader,
    AdminHeader,
  },
  props: {},
  data() {
    return {
    };
  },
  computed: {
    ...mapGetters({
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
