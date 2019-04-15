<template>
  <div>
    <el-menu
      :default-active="activeIndex"
      router
      class="el-menu-demo"
      mode="horizontal"
      @select="handleSelect"
    >
      <el-menu-item index="/">Home</el-menu-item>
      <el-menu-item index="/articles">Blog List</el-menu-item>
      <el-menu-item index="/about">About</el-menu-item>
    </el-menu>
  </div>
</template>
<script>
export default {
  name: "Header",
  components: {},
  props: {},
  data() {
    return {
      activeIndex: "/home"
    };
  },
  methods: {
    handleSelect(key, keyPath) {
      console.log(key, keyPath);
    }
  }
};
</script>
