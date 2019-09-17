<template>
  <div
    id="app"
    class="container"
  >
    <nav-header></nav-header>
    <hr />
    <router-view />
    <hr />
    <nav-footer></nav-footer>
  </div>
</template>

<script>
import NavHeader from "@/components/Header";
import NavFooter from "@/components/Footer";

export default {
  name: "app",
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
