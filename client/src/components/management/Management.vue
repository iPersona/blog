<template>
  <div class="container">
    <div class="tile is-ancestor">
      <div class="tile is-vertical is-2">
        <action-menu align="left" />
      </div>
      <div class="tile is-vertical">
        <router-view></router-view>
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