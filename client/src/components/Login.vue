<template>
  <form action="">
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">
          Login
        </p>
      </header>
      <section class="modal-card-body">
        <BField label="Username">
          <BInput
            :value="username"
            placeholder="Your username"
            required
          />
        </BField>

        <BField label="Password">
          <BInput
            type="password"
            :value="password"
            password-reveal
            placeholder="Your password"
            required
          />
        </BField>

        <BCheckbox :v-model="remember">
          Remember me
        </BCheckbox>
      </section>
      <footer class="modal-card-foot">
        <button
          class="button"
          type="button"
          @click="$parent.close()"
        >
          Close
        </button>
        <button
          class="button is-primary"
          @click="login"
        >
          Login
        </button>
      </footer>
    </div>
  </form>
</template>

<script>
import { mapMutations } from 'vuex'
import Api from "@/api.js"
import Log from "./utils/log"
import Ui from './utils/ui'
import Util from '@/utils.js'
import { LOGIN } from "@/store/modules/mutation-types.js"
import { USER } from '@/store/modules/module-names'

export default {
  name: "Login",
  components: {},
  data() {
    return {
      username: '',
      password: '',
      remember: true,
      log: new Log(this),
      ui: new Ui(this),
    };
  },
  async mounted() {
    // let api = new Api();
    // let rsp = await api.login("admin", "admin", true);
    // // let rsp = await api.login("user-1", "123456", true);
    // this.log.debug("rsp: " + JSON.stringify(rsp));
    // let token = rsp.data;
    // this.log.debug(`token: ${token}`)
    // this.login(token);
  },
  methods: {
    ...mapMutations(USER, {
      updateToken: LOGIN
    }),
    async login() {
      // reCHAPTCHA verification
      let rechaptchaToken = await this.recaptcha()
      console.log(`rechaptchaToken: ${rechaptchaToken}`)

      let api = new Api();
      // let rsp = await api.login(this.username, Util.password(this.password), this.remember);
      let rsp = await api.login("admin", Util.password("admin"), true, rechaptchaToken);
      // let rsp = await api.login("user-1", Util.password("123456"), true);
      this.log.debug("rsp: " + JSON.stringify(rsp));
      if (!Api.isSuccessResponse(rsp)) {
        this.ui.toastFail(`${rsp.detail}`)
        return
      }

      // save token
      let token = rsp.data;
      this.log.debug(`token: ${token}`)
      this.updateToken(token)

      // refresh page to make other components loading data
      window.location.reload()
    },
    async recaptcha() {
      return await this.$recaptcha('login').then((token) => {
        console.log(token) // Will print the token
        return token
      })
    }
  },
}
</script>