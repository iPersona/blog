<template>
  <div>
    <b-menu label="Menu">
      <b-menu-list>
        <b-menu-item
          label="Statistic"
          tag="router-link"
          to="/management/stat"
          icon-pack="fas"
          icon="chart-bar"
        ></b-menu-item>
        <b-menu-item
          label="Articles"
          tag="router-link"
          to="/management/articles"
          icon-pack="fas"
          icon="newspaper"
        ></b-menu-item>
        <b-menu-item
          label="Tags"
          tag="router-link"
          to="/management/tags"
          icon-pack="fas"
          icon="tags"
        ></b-menu-item>
        <b-menu-item
          label="Settings"
          icon-pack="fas"
          tag="router-link"
          to="/management/settings"
          icon="cog"
        ></b-menu-item>
      </b-menu-list>

    </b-menu>
  </div>
</template>
<script>
export default {
  name: "ActionMenu",
  components: {},
  data() {
    return {}
  },
  mounted() {

  },
  methods: {
  },
}
</script>