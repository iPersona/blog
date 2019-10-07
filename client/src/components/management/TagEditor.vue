<template>
  <form>
    <div
      class="modal-card"
      style="width: auto"
    >
      <header class="modal-card-head">
        <p class="modal-card-title">Tag: {{tagName}}</p>
      </header>
      <section class="modal-card-body">
        <b-field label="Name">
          <b-input
            type="text"
            v-model="newTagName"
            required
          >
          </b-input>
        </b-field>
      </section>
      <footer class="modal-card-foot">
        <b-button @click="$parent.close()">Cancel</b-button>
        <b-button
          class="is-primary"
          @click="modifyTag"
        >Modify</b-button>
      </footer>
    </div>
  </form>
</template>

<script>
export default {
  name: 'TagEditor',
  components: {},
  props: {
    tagId: '',
    tagName: '',
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