<template>
  <section align="left">
    <b-field label="Name">
      <b-input
        v-model="name"
        disabled
      />
    </b-field>

    <b-field label="Nick name">
      <b-input v-model="nickname" />
    </b-field>

    <b-field label="Email">
      <b-input v-model="email" />
    </b-field>

    <b-field label="Regist time">
      <b-input
        v-model="registTime"
        disabled
      />
    </b-field>

    <b-field label="Signature">
      <b-input
        v-model="sign"
        type="textarea"
        maxlength="200"
        placeholder="Tell us a little bit about yourself"
      />
    </b-field>

    <b-field>
      <p class="control">
        <button
          class="button is-primary"
          @click="save"
        >
          Update profile
        </button>
      </p>
    </b-field>

    <br>
  </section>
</template>

<script>
import { mapGetters, mapMutations } from 'vuex'
import { USER_INFO } from '@/store/modules/store-types.js'
import { UPDATE_LOGIN_DATA } from "@/store/modules/mutation-types.js"
import { USER } from '@/store/modules/module-names'
import Api from '@/api'

export default {
  name: 'Profile',
  components: {

  },
  data() {
    return {
      name: '',
      nickname: '',
      email: '',
      sign: '',
      registTime: '',
      oriUserInfo: {},
    }
  },
  computed: {
    ...mapGetters(USER, {
      userInfo: USER_INFO,
    }),
  },
  watch: {
    // userInfo is NOT available on mounted, so in order to get state immediately, use the method below
    // ref: https://juejin.im/post/5ae91fa76fb9a07aa7677543
    userInfo: {
      handler(newVal, oldVal) {
        this.oriUserInfo = newVal
        this.updateUserInfo(newVal)
      },
      immediate: true,
    }
  },
  mounted() {
  },
  methods: {
    ...mapMutations(USER, {
      updateLoginData: UPDATE_LOGIN_DATA
    }),
    updateUserInfo(userInfo) {
      this.name = userInfo.name
      this.nickname = userInfo.nickname
      this.email = userInfo.email
      this.sign = userInfo.sign
      this.registTime = userInfo.registTime
    },
    async save() {
      let api = new Api()
      let rsp = await api.editProfile({
        nickname: this.nickname,
        sign: this.sign,
        email: this.email,
      })
      if (!rsp.isSuccess()) {
        this.$getUi().toast.fail(`update profile failed: ${rsp.errorDetail()}`)
        return
      }

      // save token
      let token = rsp.data();
      this.$getLog().debug(`token: ${token}`)
      this.updateLoginData(token)

      this.$getUi().toast.success('update profile successfully!')
    }
  },
}
</script>