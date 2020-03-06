<template>
  <div class="container area">
    <div class="tile is-ancestor">
      <div class="tile is-vertical is-2">
        <ActionMenu
          align="left"
          :items="menu"
        />
      </div>
      <div class="tile is-vertical">
        <RouterView class="router-view" />
      </div>
    </div>
  </div>
</template>

<script>
import ActionMenu from "@/components/controllers/ActionMenu"

export default {
  name: "Management",
  components: { ActionMenu },
  data() {
    return {
      // action menu
      menu: [
        {
          label: 'Dashboard',
          to: '/management/stat',
          icon: 'pie-chart'
        },
        {
          label: 'Articles',
          to: '/management/articles',
          icon: 'file-text'
        },
        {
          label: 'Tags',
          to: '/management/tags',
          icon: 'tag'
        },
        {
          label: 'Settings',
          to: '/management/settings',
          icon: 'settings'
        }
      ]
    }
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