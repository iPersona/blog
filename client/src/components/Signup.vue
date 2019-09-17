<template>
  <form action="">
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">Signup</p>
      </header>
      <section class="modal-card-body">
        <b-field
          label="Username"
          align="left"
        >
          <b-input
            v-model="username"
            placeholder="Your username"
            required
            validation-message="Username can only contains letters, number and underline"
            pattern="^/w{1,20}$"
          >
          </b-input>
        </b-field>

        <b-field
          label="Password"
          align="left"
        >
          <b-input
            type="password"
            v-model="password"
            password-reveal
            placeholder="Your password"
            required
            validation-message="Password can only contains letters, numbers and underline with length between 8~16"
            pattern="^[a-zA-Z]/w{8,16}$"
          >
          </b-input>
        </b-field>

        <b-field
          label="Nickname"
          align="left"
        >
          <b-input
            v-model="nickname"
            placeholder="Your nickname"
          >
          </b-input>
        </b-field>

        <b-field
          label="Email"
          align="left"
        >
          <b-input
            type="email"
            v-model="email"
            placeholder="Your email"
            required
          >
          </b-input>
        </b-field>

        <b-field align="left">
          <b-button
            type="is-primary"
            outlined
            @click="checkEmail"
          >Verify email</b-button>
        </b-field>

        <b-field
          label="Signature"
          align="left"
        >
          <b-input
            maxlength="200"
            type="textarea"
            v-model="sign"
          ></b-input>
        </b-field>
      </section>
      <footer class="modal-card-foot">
        <button
          class="button"
          type="button"
          @click="$parent.close()"
        >Close</button>
        <button
          class="button is-primary"
          @click="signup"
        >Signup</button>
      </footer>
    </div>
  </form>
</template>

<script>
import Api from '@/api.js'
import Ui from './utils/ui.js'
import Log from './utils/log.js'
import Utils from '@/utils.js'

export default {
  name: 'signup',
  components: {

  },
  data() {
    return {
      username: "",
      password: "",
      nickname: "",
      email: "",
      sign: "",
      ui: new Ui(this),
      log: new Log(this),
    };
  },
  methods: {
    async signup() {
      this.username = "user-1";
      this.password = "123456";
      this.nickname = "user-1-nickname";
      this.email = "freedom5598@gmail.com";
      this.sign = "I'm a demo";

      let api = new Api();
      let rsp = await api.signup({
        username: this.username,
        password: Utils.password(this.password),
        nickname: this.nickname,
        email: this.email,
        sign: this.sign,
      });

      console.log("signup-resp: " + JSON.stringify(rsp));
      if (Api.isSuccessResponse(rsp)) {
        this.ui.toastSuccess('Congratulation! You have successfully create an account!')
      } else {
        this.ui.toastFail('Sorry! We are failed to create an account for you now, please try again later.');
      }
      this.$parent.close();  // close signup window
    },
    async checkEmail() {
      if (this.email === "") {
        return;
      }
      let api = new Api();
      let rsp = await api.checkUserExist(this.email);
      console.log("result: " + JSON.stringify(rsp));

      if (rsp.data === true) {
        this.ui.toastFail('Sorry! We are failed to create an account for you now, please try again later.')
      } else {
        this.ui.toastSuccess('Congratulation! This email can be used for register!');
      }
    },
  },
}
</script>