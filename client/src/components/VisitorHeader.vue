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
          <b-button
            type="is-text"
            @click="isLoginModalActive = true"
          >
            <b>Login</b>
          </b-button>
        </b-navbar-item>

        <b-navbar-item>
          <b-button
            type="is-primary"
            outlined
            @click="isSignupModalActive = true"
          ><b>Sign up</b></b-button>
        </b-navbar-item>

        <b-modal
          :active.sync="isLoginModalActive"
          has-modal-card
        >
          <login-form />
        </b-modal>

        <b-modal
          :active.sync="isSignupModalActive"
          has-modal-card
          full-screen
          :can-cancel="false"
        >
          <signup-form></signup-form>
        </b-modal>
      </template>

    </b-navbar>
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