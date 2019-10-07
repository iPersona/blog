<template>
  <div
    class="modal-card"
    style="width: auto"
  >
    <header class="modal-card-head">
      <p class="modal-card-title">Article Editor</p>
    </header>
    <section class="modal-card-body">
      <b-field
        horizontal
        label="Title"
      >
        <b-input
          name="title"
          expanded
          v-model="title"
          required
          validation-message="Title can not be empty"
          pattern="[^\s]*"
        ></b-input>
      </b-field>

      <b-field
        horizontal
        label="Tags"
      >
        <b-taginput
          v-model="tags"
          :data="filteredTags"
          autocomplete
          allow-new
          field="tag"
          icon="label"
          placeholder="Add a tag"
          @typing="getFilteredTags"
          @remove="deselectTag"
          @add="addNewTag"
        >
        </b-taginput>
      </b-field>

      <b-field
        horizontal
        label="Content"
      >
        <editor
          align="left"
          v-model="content"
          :previewStyle="previewStyle"
        />
      </b-field>
      <b-field
        horizontal
        label="Publish"
      >
        <p class="control">
          <b-switch :value="publish" />
        </p>
      </b-field>
    </section>
    <footer class="modal-card-foot">
      <!-- <b-field horizontal> -->
      <p class="buttons">
        <b-button
          type="is-danger"
          @click="cancel"
        >Cancel</b-button>
        <b-button
          type="is-primary"
          @click="publishArticle"
        >Publish</b-button>
      </p>
      <!-- </b-field> -->
    </footer>
  </div>
</template>

<script>
import 'tui-editor/dist/tui-editor.css'
import 'tui-editor/dist/tui-editor-contents.css'
import 'codemirror/lib/codemirror.css'
import { Editor } from '@toast-ui/vue-editor'

import Api from '@/api.js'
import Log from './utils/log.js'
import Ui from './utils/ui.js'

import { EventBus } from '@/event-bus.js';

// import Editor from '@toast-ui/vue-editor'

export default {
  name: 'ArticleEditor',
  data() {
    return {
      title: '',
      content: '',
      previewStyle: 'vertical',
      oldTags: [],
      tags: [], // final selected tag
      filteredTags: [], // tags show in autocomplete panel
      availableTags: [],
      deselectedTags: [],
      publish: true,
      log: new Log(this),
      ui: new Ui(this),
    };
  },
  props: {
    articleId: String,
    isCreateNew: Boolean,
  },
  components: {
    'editor': Editor
  },
  async mounted() {
    await this.getArticle()
    await this.getTags()
  },
  methods: {
    async getArticle() {
      if (this.articleId === undefined || this.articleId === '') {
        return;
      }

      let api = new Api();
      let rsp = await api.visitorViewArticle(this.articleId);
      if (!Api.isSuccessResponse(rsp)) {
        return;
      }
      this.log.debug(`rsp: ${JSON.stringify(rsp)}`);
      let article = rsp.data;
      this.title = article.title
      this.content = article.content
      this.tags = this.genTagObjectArray(article.tags, article.tags_id)
      this.oldTags = this.genTagObjectArray(article.tags, article.tags_id)
    },
    async getTags() {
      let api = new Api()
      let rsp = await api.getTags()
      this.log.debug(`get tags: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        this.log.error(`get tags failed: ${rsp.detail}`)
        return
      }
      this.availableTags = rsp.data;
    },
    getFilteredTags(text) {
      this.filteredTags = this.availableTags.filter((option) => {
        return option.tag
          .toString()
          .toLowerCase()
          .indexOf(text.toLowerCase()) >= 0
      })
    },
    async publishArticle() {
      if (this.title === '') {
        this.ui.toastFail('Title can not be empty!')
        return
      }

      let existTags = this.tags.filter(t => {
        return typeof (t) === 'object'
      }).map(t => {
        return t.id
      })
      let newTags = this.tags.filter(t => {
        return typeof (t) === 'string'
      })
      this.log.debug(`existTags: ${existTags}`)
      this.log.debug(`newTags: ${newTags}`)
      let api = new Api()
      var rsp;
      if (this.isCreateNew) {
        rsp = await api.createArticle(this.title, this.content, existTags, newTags, this.publish)
      } else {
        let classifiedTags = this.classifyTags()
        rsp = await api.editArticle(this.articleId, this.title, this.content, classifiedTags.newChoiceAlreadyExistTags, classifiedTags.deselectedTags, classifiedTags.newTags)
      }
      if (!Api.isSuccessResponse(rsp)) {
        this.ui.toastFail(`Failed to ${this.isCreateNew ? "create" : "update"} article: ${rsp.detail}`)
        return
      }

      this.ui.toastSuccess('Article is successfully published!')

      // reload article to update
      EventBus.$emit('reload-data')

      // // go back to previous view
      // this.$router.go(-1)
      this.$parent.close()
    },
    cancel() {
      this.$parent.close()
    },
    classifyTags() {
      return {
        newTags: this.tags.filter(t => {
          return typeof (t) === 'string'
        }),
        newChoiceAlreadyExistTags: this.tags.filter(t => {
          if (typeof (t) === 'string') {
            return false
          }

          if (this.oldTags.length === 0) {
            return true
          }

          return this.oldTags.some(ot => {
            return ot.id !== t.id
          })
        }).map(t => {
          return t.id
        }),
        deselectedTags: this.deselectedTags.map(t => {
          return t.id
        }),
      }
    },
    genTagObjectArray(tags, tagsId) {
      var tagObjects = []
      if (tags.length !== tagsId.length) {
        return tagObjects
      }

      for (var i = 0; i != tags.length; i++) {
        if (tags[i] === null || tagsId[i] === null) {
          continue
        }

        tagObjects.push({
          tag: tags[i],
          id: tagsId[i]
        })
      }
      return tagObjects
    },
    deselectTag(value) {
      if (typeof (value) === 'string') {
        return
      }
      this.deselectedTags.push(value)
    },
    addNewTag(value) {
      if (this.availableTags.some(t => {
        return t.tag.toLowerCase() === value.toLowerCase()
      })) {
        this.tags = this.tags.filter(t => {
          if (typeof (t) === 'string') {
            return t.toLowerCase() !== value.toLowerCase()
          }
          return true
        })
      }
    }
  },
}
</script>