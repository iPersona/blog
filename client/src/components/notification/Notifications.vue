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
  name: 'Notifications',
  components: {
    ActionMenu,
  },
  data() {
    return {
      menu: [
        {
          label: 'Comments',
          to: { name: 'comment-notification' },
          icon: 'message-circle'
        },
      ]
    }
  },
  mounted() {
    // set default view to comment notification 
    this.setupDefaultView()
  },
  methods: {
    setupDefaultView() {
      this.$router.push({
        name: 'comment-notification'
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