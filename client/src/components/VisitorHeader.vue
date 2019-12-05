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
          <BButton
            type="is-text"
            @click="isLoginModalActive = true"
          >
            <b>Login</b>
          </BButton>
        </BNavbarItem>

        <BNavbarItem>
          <BButton
            type="is-primary"
            outlined
            @click="isSignupModalActive = true"
          >
            <b>Sign up</b>
          </BButton>
        </BNavbarItem>

        <BModal
          :active.sync="isLoginModalActive"
          has-modal-card
        >
          <LoginForm />
        </BModal>

        <BModal
          :active.sync="isSignupModalActive"
          has-modal-card
          full-screen
          :can-cancel="false"
        >
          <SignupForm />
        </BModal>
      </template>
    </BNavbar>
  </div>
</template>

<script>
import LoginForm from '@/components/Login.vue'
import SignupForm from '@/components/Signup.vue'

export default {
  name: 'VisitorHeader',
  components: {
    LoginForm,
    SignupForm
  },
  data() {
    return {
      isLoginModalActive: false,
      isSignupModalActive: false,
    }
  },
  methods: {

  },
}
</script>