<template>
  <div
    v-if="isLogin && isAdmin"
    class="container"
  >
    <form>
      <section>
        <!-- search key -->
        <BField grouped>
          <BInput
            v-model="searchKey"
            type="search"
            placeholder="Search..."
            expanded
            icon-pack="fas"
            icon="search"
            @input="searchTag"
          />

          <BButton
            type="is-primary"
            outlined
            @click="openAddTagPanel"
          >
            Add tags
          </BButton>
          <!-- tag edit panel -->
          <BModal
            :active.sync="isAddTagPanelOpen"
            has-modal-card
          >
            <AddTagPanel @addNewTags="addNewTags" />
          </BModal>
        </BField>

        <!-- exist tags -->
        <TagPanel
          title="Exist Tags"
          :tags="filteredTags"
          :enable-tag-edit="true"
          placeholder="No tags currently"
          @deleteTag="deleteTag"
          @updateTag="updateTag"
        />

        <!-- tips -->
        <div align="left">
          <span
            align="left"
            class="tips-head"
          >
            TIPS:
          </span>
          <ul
            align="left"
            class="tips-item"
          >
            <li>double click to edit tags</li>
            <li>click delete button to delete tag</li>
          </ul>
        </div>
        <br>

        <!-- modified tags -->
        <TagPanel
          title="Modified Tags"
          :tags="modified_tags"
          placeholder="No modified tags currently"
          tag-type="is-warning"
          @deleteTag="deleteModifiedTag"
        />

        <!-- added tags -->
        <TagPanel
          title="Added Tags"
          :tags="added_tags"
          placeholder="No new added tags currently"
          tag-type="is-success"
          @deleteTag="deleteAddedTag"
        />

        <!-- deleted tags -->
        <TagPanel
          title="Deleted Tags"
          :tags="deleted_tags"
          placeholder="No deleted tags currently"
          tag-type="is-danger"
          @deleteTag="deleteDeletedTag"
        />

        <BField grouped>
          <BButton
            type="is-primary"
            @click="updateTags"
          >
            Update tags
          </BButton>
        </BField>
      </section>
    </form>
  </div>
  <div v-else />
</template>

<script>
import { mapGetters } from 'vuex'
import { IS_LOGIN, IS_ADMIN } from '@/store/modules/store-types.js'
import Api from '@/api'
import AddTagPanel from './AddTagPanel'
import TagPanel from './TagPanel'
import { NOT_CHANGED, MODIFIED, ADDED } from './tag-status'
import { USER } from '@/store/modules/module-names'

export default {
  name: 'Tags',
  components: {
    AddTagPanel,
    TagPanel,
  },
  data() {
    return {
      tags: [],
      filteredTags: [],
      deleted_tags: [],
      added_tags: [],
      modified_tags: [],
      isEditPanelOpen: false,
      isAddTagPanelOpen: false,
      tagId: '',
      tagName: '',
      searchKey: '',
    }
  },
  computed: {
    ...mapGetters(USER, {
      isLogin: IS_LOGIN,
      isAdmin: IS_ADMIN,
    }),

  },
  watch: {
    searchKey(newVal, oldVal) {
      if (newVal === '') {
        this.filteredTags = this.tags;
      }
    }
  },
  async mounted() {
    await this.reloadTags()
  },
  methods: {
    async reloadTags() {
      this.tags = await this.getTags()
      this.filteredTags = this.tags
      this.deleted_tags = []
      this.added_tags = []
      this.modified_tags = []
    },
    async getTags() {
      let api = new Api()
      let rsp = await api.getTags();
      this.$getLog().debug(`tags: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        return
      }
      let tags = rsp.data
      return this.addStatus(tags)
    },
    addStatus(tags) {
      for (var i in tags) {
        tags[i]['isActive'] = true
      }
      return tags
    },
    openTagEditor(tag) {
      this.isEditPanelOpen = true
      this.tagId = tag.id
      this.tagName = tag.tag
    },
    tagType(tag) {
      if (this.added_tags.filter(item => {
        return item.tag === tag.tag
      }).length > 0) {
        return 'is-success'
      }

      if (this.deleted_tags.filter(item => {
        return item.tag === tag.tag
      }).length > 0) {
        return 'is-danger'
      }

      if (this.modified_tags.filter(item => {
        return item.tag === tag.tag
      }).length > 0) {
        return 'is-warning'
      }

      return 'is-primary'
    },
    updateTag(id, name, isModified) {
      if (!isModified) {
        return
      }

      let tag = this.tags.filter(item => {
        return item.id === id
      }).map(item => {
        item.origin = item.tag
        item.tag = name
        return item
      })[0];
      this.modified_tags.push(tag)
      this.filteredTags = this.filteredTags.filter(item => {
        return item.id !== tag.id
      })
    },
    addNewTags(nameArray) {
      console.log(`addNewTags`)
      let tags = nameArray.map(item => {
        return {
          tag: item,
          isActive: true
        }
      })
      console.log(`newAddedTags: ${JSON.stringify(tags)}`)
      this.added_tags = this.added_tags.concat(tags)
      console.log(`added_tags: ${JSON.stringify(this.added_tags)}`)
    },
    searchTag() {
      if (this.searchKey === '') {
        this.filteredTags = this.tags.filter(item => {
          return this.modified_tags.filter(t => {
            return item.id === t.id
          }).length <= 0
            && this.deleted_tags.filter(t => {
              return item.id === t.id
            }).length <= 0
        })
        return
      }
      this.filteredTags = this.tags.filter(item => {
        let t = item.tag.toString().toLowerCase()
        let k = this.searchKey.toString().toLowerCase()
        return t.indexOf(k) >= 0
      })
      console.log(`this.filtredTags: ${JSON.stringify(this.filteredTags)}`)
    },
    deleteTag(tag) {
      this.filteredTags = this.filteredTags.filter(item => {
        return item.id !== tag.id
      })
      this.deleted_tags.push(tag)
    },
    deleteModifiedTag(tag) {
      this.modified_tags = this.modified_tags.filter(item => {
        return item.id !== tag.id
      })

      tag.tag = tag.origin  // recover original name
      this.filteredTags.push(tag)
    },
    deleteAddedTag(tag) {
      this.added_tags = this.added_tags.filter(item => {
        return item.tag !== tag.tag
      })
    },
    deleteDeletedTag(tag) {
      this.deleted_tags = this.deleted_tags.filter(item => {
        return item.id !== tag.id
      })
      this.filteredTags.push(tag)
    },
    openAddTagPanel() {
      this.isAddTagPanelOpen = true
    },
    async updateTags() {
      let modified_tags = this.modified_tags.map(item => {
        return {
          id: item.id,
          tag: item.tag,
        }
      })
      let added_tags = this.added_tags.map(item => {
        return item.tag
      })
      let deleted_tags = this.deleted_tags.map(item => {
        return item.id
      })

      let api = new Api()
      let rsp = await api.updateTags(modified_tags, added_tags, deleted_tags)
      console.log(`data: ${JSON.stringify(rsp)}`)
      if (Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.success('tags are successfully updated!')
      } else {
        this.$getUi().toast.fail(`update tags failed(${rsp.code}): ${rsp.detail}`)
      }

      // reload tags
      this.reloadTags()
    }
  },
}
</script>

<style scoped>
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