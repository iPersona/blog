<template>
  <form action="">
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">
          Sign Up
        </p>
      </header>
      <section class="modal-card-body">
        <BField
          label="Username"
          align="left"
        >
          <BInput
            v-model="username"
            placeholder="Your username"
            required
            validation-message="Username can only contains letters, number and underline"
            pattern="^/w{1,20}$"
          />
        </BField>

        <BField
          label="Password"
          align="left"
        >
          <BInput
            v-model="password"
            type="password"
            password-reveal
            placeholder="Your password"
            required
            validation-message="Password can only contains letters, numbers and underline with length between 8~16"
            pattern="^[a-zA-Z]/w{8,16}$"
          />
        </BField>

        <BField
          label="Nickname"
          align="left"
        >
          <BInput
            v-model="nickname"
            placeholder="Your nickname"
          />
        </BField>

        <BField
          label="Email"
          align="left"
        >
          <BInput
            v-model="email"
            type="email"
            placeholder="Your email"
            required
          />
        </BField>

        <BField align="left">
          <BButton
            type="is-primary"
            outlined
            @click="checkEmail"
          >
            Verify email
          </BButton>
        </BField>

        <BField
          label="Signature"
          align="left"
        >
          <BInput
            v-model="sign"
            maxlength="200"
            type="textarea"
          />
        </BField>
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
          @click="signup"
        >
          Signup
        </button>
      </footer>
    </div>
  </form>
</template>

<script>
import Api from '@/api.js'
import Utils from '@/utils.js'

export default {
  name: 'Signup',
  components: {

  },
  data() {
    return {
      username: "",
      password: "",
      nickname: "",
      email: "",
      sign: "",
    };
  },
  methods: {
    async signup() {
      this.username = "user-4";
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
      if (rsp.isSuccess()) {
        this.$getUi().toast.success(`An verification email is send to ${this.email}. Please verify your email to active your account!`)
      } else {
        this.$getUi().toast.fail('Sorry! We are failed to create an account for you now, please try again later.');
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

      if (rsp.data() === true) {
        this.$getUi().toast.fail('Sorry! We are failed to create an account for you now, please try again later.')
      } else {
        this.$getUi().toast.success('Congratulation! This email can be used for register!');
      }
    },
  },
}
</script>