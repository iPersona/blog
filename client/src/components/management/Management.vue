<template>
  <div class="container">
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
import ActionMenu from './ActionMenu'

export default {
  name: 'Management',
  components: { ActionMenu },
  data() {
    return {}
  },
  mounted() {

  },
  methods: {
  },
}
</script>

<style scoped>
.router-view {
  margin-left: 20px;
}
</style>