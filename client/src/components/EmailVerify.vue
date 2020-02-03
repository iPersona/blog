<template>
  <div>
    <span class="info">
      Verifying account ...
    </span>
    <br>
    <br>
    <half-circle-spinner
      style="margin:auto"
      :animation-duration="1000"
      :size="40"
      color="#7957d5"
    />
  </div>
</template>

<script>
import { mapMutations } from 'vuex'
import Api from "@/api.js"
import Util from '@/utils.js'
import { UPDATE_TOKEN } from "@/store/modules/mutation-types.js"
import { USER } from '@/store/modules/module-names'
import { HalfCircleSpinner } from 'epic-spinners'

export default {
  name: "EmailVerify",
  components: {
    HalfCircleSpinner
  },
  data() {
    return {
      token: this.$route.params.token,
    }
  },
  async mounted() {
    console.log(`token: ${this.token}`)
    this.verify()
  },
  methods: {
    ...mapMutations(USER, {
      updateToken: UPDATE_TOKEN
    }),
    async verify() {
      let api = new Api();
      let rsp = await api.verify(this.token)
      this.$getLog().debug("rsp: " + JSON.stringify(rsp))
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`${rsp.detail}`)
        return
      }

      // save token
      let token = rsp.data;
      this.$getLog().debug(`token: ${token}`)
      this.updateToken(token)

      // redirect to home page
      this.$router.replace({ name: 'articles' })
    },
  },
}
</script>

<style scoped>
.info {
  font-weight: bold;
  font-size: large;
}
</style>