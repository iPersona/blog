<template>
  <div
    id="app"
    class="container"
  >
    <NavHeader />
    <RouterView />
    <NavFooter />
  </div>
</template>

<script>
import { mapMutations } from 'vuex'
import { LOAD_USER, LOAD_TAG } from '@/store/modules/mutation-types'
import { USER, TAG } from '@/store/modules/module-names'

import NavHeader from "@/components/Header";
import NavFooter from "@/components/Footer";

export default {
  name: "App",
  components: {
    NavHeader,
    NavFooter
  },
  data() {
    return {
      tab: 1
    };
  },
  mounted() {
    console.log(`App mounted`)
    this.loadAppData()
  },
  methods: {
    ...mapMutations(USER, {
      loadUser: LOAD_USER
    }),
    ...mapMutations(TAG, {
      loadTag: LOAD_TAG
    }),
    loadAppData() {
      this.loadUser()
      this.loadTag()
    }
  },
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
@import "https://use.fontawesome.com/releases/v5.2.0/css/all.css";
</style>

<style scoped src="buefy/dist/buefy.css">
</style>