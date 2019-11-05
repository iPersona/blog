<template>
  <form>
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">
          Add Tags
        </p>
      </header>
      <section class="modal-card-body">
        <BTaginput
          v-model="tags"
          ellipsis
          icon="label"
          placeholder="Add a tag"
        />
      </section>
      <footer class="modal-card-foot">
        <BButton @click="$parent.close()">
          Cancel
        </BButton>
        <BButton
          class="is-primary"
          @click="addNewTag"
        >
          Done
        </BButton>
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