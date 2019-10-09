<template>
  <div
    class="container"
    v-if="isLogin && isAdmin"
  >
    <form>
      <section>
        <!-- search key -->
        <b-field grouped>
          <b-input
            type="search"
            v-model="searchKey"
            placeholder="Search..."
            expanded
            icon-pack="fas"
            icon="search"
            @input="serchTag"
          />

          <b-button
            type="is-primary"
            outlined
            @click="openAddTagPanel"
          >Add tags</b-button>
          <!-- tag edit panel -->
          <b-modal
            :active.sync="isAddTagPanelOpen"
            has-modal-card
          >
            <add-tag-panel v-on:addNewTags="addNewTags" />
          </b-modal>
        </b-field>

        <!-- exist tags -->
        <h1 class="label">Exist Tags</h1>
        <b-field
          grouped
          class="taglist"
        >
          <b-taglist>
            <b-tag
              type="is-primary"
              size="is-medium"
              closable
              aria-close-label="Close tag"
              v-for="tag in filteredTags"
              v-bind:key="tag.uuid"
              v-if="tag.isActive"
              @close="deleteTag(tag)"
              @dblclick.native="openTagEditor(tag)"
            >
              {{tag.tag}}
            </b-tag>
          </b-taglist>
          <span
            align="right"
            class="tags-number"
          >{{filteredTags.length}}</span>

          <!-- tag edit panel -->
          <b-modal
            :active.sync="isEditPanelOpen"
            has-modal-card
          >
            <tag-editor
              :tagId="tagId"
              :tagName="tagName"
              v-on:updateTag="updateTag"
            />
          </b-modal>
        </b-field>
        <b-field align="left">
          <span class="tips-head">TIPS:</span>
        </b-field>
        <b-field>
          <ul
            align="left"
            class="tips-item"
          >
            <li>double click to edit tags</li>
            <li>click delete button to delete tag</li>
          </ul>
        </b-field>
        <br />

        <!-- modified tags -->
        <h1 class="label">Modified Tags</h1>
        <b-field
          grouped
          class="taglist"
        >
          <b-taglist>
            <b-tag
              type="is-warning"
              size="is-medium"
              closable
              aria-close-label="Close tag"
              v-for="tag in modified_tags"
              v-bind:key="tag.uuid"
              v-if="tag.isActive"
              @close="deleteModifiedTag(tag)"
            >
              {{tag.tag}}
            </b-tag>
          </b-taglist>
          <span
            align="right"
            class="tags-number"
          >{{modified_tags.length}}</span>
        </b-field>

        <!-- added tags -->
        <h1 class="label">Added Tags</h1>
        <b-field
          grouped
          class="taglist"
        >
          <b-taglist>
            <b-tag
              type="is-success"
              size="is-medium"
              closable
              aria-close-label="Close tag"
              v-for="tag in added_tags"
              v-bind:key="tag.uuid"
              v-if="tag.isActive"
              @close="deleteAddedTag(tag)"
            >
              {{tag.tag}}
            </b-tag>
          </b-taglist>
          <span
            align="right"
            class="tags-number"
          >{{added_tags.length}}</span>
        </b-field>

        <!-- deleted tags -->
        <h1 class="label">Deleted Tags</h1>
        <b-field
          grouped
          class="taglist"
        >
          <b-taglist>
            <b-tag
              type="is-danger"
              size="is-medium"
              closable
              aria-close-label="Close tag"
              v-for="tag in deleted_tags"
              v-bind:key="tag.uuid"
              v-if="tag.isActive"
              @close="deleteDeletedTag(tag)"
            >
              {{tag.tag}}
            </b-tag>
          </b-taglist>
          <span
            align="right"
            class="tags-number"
          >{{deleted_tags.length}}</span>
        </b-field>

        <b-field grouped>
          <b-button
            type="is-primary"
            @click="submit"
          >Submit</b-button>
        </b-field>
      </section>
    </form>
  </div>
  <div v-else>

  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import { IS_LOGIN, IS_ADMIN } from '@/store-types.js'
import Api from '@/api'
import Ui from '@/components/utils/ui'
import Log from '@/components/utils/log'
import TagEditor from './TagEditor'
import AddTagPanel from './AddTagPanel'
import { NOT_CHANGED, MODIFIED, ADDED } from './tag-status'

export default {
  name: 'Tags',
  data() {
    return {
      tags: [],
      filteredTags: [],
      deleted_tags: [],
      added_tags: [],
      modified_tags: [],
      log: new Log(this),
      ui: new Ui(this),
      isEditPanelOpen: false,
      isAddTagPanelOpen: false,
      tagId: '',
      tagName: '',
      searchKey: '',
    }
  },
  components: {
    TagEditor,
    AddTagPanel
  },
  computed: {
    ...mapGetters({
      isLogin: IS_LOGIN,
      isAdmin: IS_ADMIN,
    }),

  },
  watch: {
    searchKey(oldVal, newVal) {
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
      this.log.debug(`tags: ${JSON.stringify(rsp)}`)
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
    serchTag() {
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
    async submit() {
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
        this.ui.toastSuccess('tags are successfully updated!')
      } else {
        this.ui.toastFail(`update tags failed(${rsp.code}): ${rsp.detail}`)
      }

      // reload tags
      this.reloadTags()
    }
  },
}
</script>

<style scoped>
h1.label {
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

.tips-head {
  font-size: small;
  color: silver;
}

.tips-item {
  font-size: smaller;
  color: silver;
}
</style>