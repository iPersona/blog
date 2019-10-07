<template>
  <form>
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">Add Tags</p>
      </header>
      <section class="modal-card-body">
        <b-taginput
          v-model="tags"
          ellipsis
          icon="label"
          placeholder="Add a tag"
        >
        </b-taginput>
      </section>
      <footer class="modal-card-foot">
        <b-button @click="$parent.close()">Cancel</b-button>
        <b-button
          class="is-primary"
          @click="addNewTag"
        >Done</b-button>
      </footer>
    </div>
  </form>
</template>
<script>
export default {
  name: 'AddTagPanel',
  components: {},
  data() {
    return {
      tags: []
    }
  },
  mounted() {

  },
  methods: {
    addNewTag() {
      console.log(`addNewTag-tags: ${this.tags}`)
      this.$emit('addNewTags', this.tags)
      this.$parent.close()
    }
  },
}
</script>