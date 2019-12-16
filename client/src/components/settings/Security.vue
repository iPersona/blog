<template>
  <section align="left">
    <b-field label="Old password">
      <b-input
        v-model="oldPassword"
        type="password"
        password-reveal
      />
    </b-field>

    <b-field label="New password">
      <b-input
        v-model="newPassword"
        type="password"
        password-reveal
      />
    </b-field>

    <b-field label="Confirm new password">
      <b-input
        v-model="confirmNewPassword"
        type="password"
        password-reveal
      />
    </b-field>

    <b-field>
      <p class="control">
        <button
          class="button is-primary"
          @click="updatePassword"
        >
          Update password
        </button>
      </p>
    </b-field>

    <br>
  </section>
</template>

<script>
import Api from '@/api'
import Util from '@/utils'

export default {
  name: 'Security',
  components: {

  },
  data() {
    return {
      oldPassword: '',
      newPassword: '',
      confirmNewPassword: '',
    }
  },
  mounted() {

  },
  methods: {
    async updatePassword() {
      if (this.newPassword !== this.confirmNewPassword) {
        this.$getUi().toast.fail("Password confirmation doesn't match the password")
        return
      }

      let api = new Api()
      let rsp = await api.updatePassword(Util.password(this.oldPassword), Util.password(this.newPassword))
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`${rsp.detail}`)
        return
      }
      this.$getUi().toast.success('Password is successfully updated')
    }
  },
}
</script>