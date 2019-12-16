<template>
  <div class="container area">
    <div class="tile is-ancestor">
      <div class="tile is-vertical is-2">
        <PersonalSettingMenu align="left" />
      </div>
      <div class="tile is-vertical">
        <RouterView class="router-view" />
      </div>
    </div>
  </div>
</template>

<script>
import PersonalSettingMenu from './PersonalSettingsMenu'
export default {
  name: 'PersonalSettings',
  components: {
    PersonalSettingMenu,
  },
  data() {
    return {}
  },
  mounted() {
    // set default view to profile settings
    this.setupDefaultView()
  },
  methods: {
    setupDefaultView() {
      this.$router.push({
        name: 'profile'
      })
    }
  },
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