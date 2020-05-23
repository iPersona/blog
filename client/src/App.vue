<template>
  <div
    id="app"
    class="container"
  >
    <NavHeader />
    <RouterView class="router-view" />
    <br><br>
    <NavFooter />
  </div>
</template>

<script>
import NavHeader from "@/components/Header"
import NavFooter from "@/components/Footer"

import Api from "@/api"
import { mapMutations } from 'vuex'
import { UPDATE_LOGIN_DATA, CLEAR_STATE } from "@/store/modules/mutation-types.js"
import { USER } from '@/store/modules/module-names'

export default {
  name: "App",
  components: {
    NavHeader,
    NavFooter
  },
  data() {
    return {
    }
  },
  created() {
    // get user info from server
    this.loadUserData()
  },
  mounted() {
    console.log(`App mounted`);
  },
  methods: {
    ...mapMutations(USER, {
      updateLoginData: UPDATE_LOGIN_DATA,
      resetAppData: CLEAR_STATE,
    }),
    async loadUserData() {
      let api = new Api()
      let rsp = await api.userData()
      if (!rsp.isSuccess()) {
        // clear cached vuex data
        this.resetAppData()
        return
      }

      this.updateLoginData(rsp.data())
    }
  }
};
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
@import "https://cdn.materialdesignicons.com/2.5.94/css/materialdesignicons.min.css";
/* @import "https://use.fontawesome.com/releases/v5.2.0/css/all.css"; */

.router-view {
  margin-left: 15%;
  margin-right: 15%;
}
</style>

<style scoped src="buefy/dist/buefy.css">
</style>