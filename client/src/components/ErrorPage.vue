<template>
  <div>
    <b-message
      :title="title"
      type="is-danger"
      aria-close-label="Close message"
      :closable="false"
    >
      {{ detail }}
    </b-message>
  </div>
</template>

<script>
export default {
  name: 'ErrorPage',
  components: {},
  data() {
    return {
      title: this.$route.query.title,
      detail: this.$route.query.detail,
    }
  },
  mounted() {

  },
  methods: {

  },
}
</script>