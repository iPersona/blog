<template>
  <div class="container area">
    <div class="tile is-ancestor">
      <div class="tile is-vertical is-2">
        <ActionMenu align="left" />
      </div>
      <div class="tile is-vertical">
        <RouterView class="router-view" />
      </div>
    </div>
  </div>
</template>

<script>
import ActionMenu from "./ActionMenu"

export default {
  name: "Management",
  components: { ActionMenu },
  data() {
    return {}
  },
  mounted() {
    // set default view to dashboard
    this.setupDefaultView()
  },
  methods: {
    setupDefaultView() {
      this.$router.push({
        name: 'stat'
      })
    }
  }
}
</script>

<style scoped>
.router-view {
  margin-left: 20px;
}

.area {
  margin-bottom: 50px;
}
</style>