<template>
  <div id="app">
    <nav-header></nav-header>
    <router-view/>
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
</style>
