<template>
  <div class="panel">
    <div
      class="panel-heading"
      align="left"
    >
      <strong>
        {{ title }}
      </strong>
    </div>
    <div class="panel-block is-relative">
      <div
        v-if="tags.length === 0"
        class="tags-placeholder"
      >
        {{ placeholder }}
      </div>
      <BTaglist v-else>
        <BTag
          v-for="tag in tags"
          :key="tag.uuid"
          :type="tagType"
          size="is-medium"
          closable
          aria-close-label="Close tag"
          @close="deleteTag(tag)"
          @dblclick.native="openTagEditor(tag)"
        >
          {{ tag.tag }}
        </BTag>
      </BTaglist>
      <span
        align="right"
        class="tags-number"
      >
        {{ tags.length }}
      </span>

      <!-- tag edit panel -->
      <BModal
        :active.sync="isEditPanelOpen"
        has-modal-card
      >
        <TagEditor
          :tag-id="tagId"
          :tag-name="tagName"
          @updateTag="updateTag"
        />
      </BModal>
      <!-- </BField> -->
    </div>
  </div>
</template>

<script>
import TagEditor from './TagEditor'

export default {
  name: 'TagPanel',
  components: {
    TagEditor,
  },
  props: {
    title: {
      type: String,
      default: ''
    },
    tags: {
      type: Array,
      default: () => []
    },
    placeholder: {
      type: String,
      default: ''
    },
    enableTagEdit: {
      type: Boolean,
      default: false
    },
    tagType: {
      type: String,
      default: "is-primary"
    }
  },
  data() {
    return {
      isEditPanelOpen: false,
      tagId: '',
      tagName: ''
    }
  },
  mounted() {
    console.log(`type: ${this.tagType}`)
  },
  methods: {
    deleteTag(tag) {
      this.$emit('deleteTag', tag)
    },
    updateTag(id, name, isModified) {
      this.$emit('updateTag', id, name, isModified)
    },
    openTagEditor(tag) {
      if (!this.enableTagEdit) {
        return;
      }

      this.isEditPanelOpen = true
      this.tagId = tag.id
      this.tagName = tag.tag
    },
  },
}
</script>

<style scoped>
.title {
  font-size: x-large;
  text-align: left;
  color: gray;
}

.taglist {
  border-radius: 6px;
  border: 1px solid gainsboro;
  padding: 15px;
  position: relative;
}

.tags-number {
  position: absolute;
  bottom: 5px;
  right: 10px;
  font-size: smaller;
  color: lightgray;
}

.tags-placeholder {
  font-size: large;
  color: gray;
}

.tips-head {
  font-size: small;
  font-weight: bold;
  color: silver;
  white-space: nowrap;
}

.tips-item {
  font-size: smaller;
  color: silver;
}
</style>