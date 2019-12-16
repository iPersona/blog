<template>
  <form>
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">
          Tag: {{ tagName }}
        </p>
      </header>
      <section class="modal-card-body">
        <BField label="Name">
          <BInput
            v-model="newTagName"
            type="text"
            required
          />
        </BField>
      </section>
      <footer class="modal-card-foot">
        <BButton @click="$parent.close()">
          Cancel
        </BButton>
        <BButton
          class="is-primary"
          @click="modifyTag"
        >
          Modify
        </BButton>
      </footer>
    </div>
  </form>
</template>

<script>
export default {
  name: 'TagEditor',
  components: {},
  props: {
    tagId: {
      type: String,
      default: ''
    },
    tagName: {
      type: String,
      default: ''
    },
  },
  data() {
    return {
      newTagName: this.tagName,
    }
  },
  mounted() {

  },
  methods: {
    modifyTag() {
      this.$emit('updateTag', this.tagId, this.newTagName, this.newTagName !== this.tagName)
      this.$parent.close()
    }
  },
}
</script>