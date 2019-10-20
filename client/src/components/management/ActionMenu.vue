<template>
  <div>
    <BMenu label="Menu">
      <BMenuList>
        <BMenuItem
          label="Statistic"
          tag="router-link"
          to="/management/stat"
          icon-pack="fas"
          icon="chart-bar"
        />
        <BMenuItem
          label="Articles"
          tag="router-link"
          to="/management/articles"
          icon-pack="fas"
          icon="newspaper"
        />
        <BMenuItem
          label="Tags"
          tag="router-link"
          to="/management/tags"
          icon-pack="fas"
          icon="tags"
        />
        <BMenuItem
          label="Settings"
          icon-pack="fas"
          tag="router-link"
          to="/management/settings"
          icon="cog"
        />
      </BMenuList>
    </BMenu>
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