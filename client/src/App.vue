<template>
  <div id="app">
    <b-navbar
      fixed="top"
      toggleable="md"
      type="dark"
      variant="primary"
    >
      <b-navbar-brand href="#">USDKTest</b-navbar-brand>
      <b-navbar-nav>
        <!-- [导航选项激活问题解决](https://stackoverflow.com/a/44679936/10366378) -->
        <b-nav-item
          href="#/"
          v-bind:active="tab === 1"
          v-on:click="tab = 1"
        >
          主页
        </b-nav-item>
        <b-nav-item
          href="#/files"
          v-bind:active="tab === 2"
          v-on:click="tab = 2"
        >
          文件
        </b-nav-item>
        <b-nav-item
          href="#/debug"
          v-bind:active="tab === 3"
          v-on:click="tab = 3"
        >
          测试
        </b-nav-item>
        <!-- <b-nav-item
          href="#/result"
          v-bind:active="tab === 4"
          v-on:click="tab = 4"
        >
          结果
        </b-nav-item> -->
      </b-navbar-nav>
    </b-navbar>
    <hr />
    <router-view />
  </div>
</template>

<script>
  export default {
    name: 'app',
    components: {
    },
    data() {
      return {
        tab: 1
      }
    }
  }
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
