<template>
  <div
    class="modal-card"
    style="width: auto"
  >
    <header class="modal-card-head">
      <p class="modal-card-title">
        Article Editor
      </p>
    </header>
    <section class="modal-card-body">
      <BField
        horizontal
        label="Title"
      >
        <BInput
          v-model="title"
          name="title"
          expanded
          required
          validation-message="Title can not be empty"
          pattern="[^\s]*"
        />
      </BField>

      <BField
        horizontal
        label="Tags"
      >
        <BTaginput
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
        />
      </BField>

      <BField
        horizontal
        label="Content"
      >
        <MarkdownEditor
          ref="editor"
          align="left"
          :initial-value="content"
          toolbar-style="Article"
        />
      </BField>
      <BField
        horizontal
        label="Publish"
      >
        <p class="control">
          <BSwitch :value="publish" />
        </p>
      </BField>
    </section>
    <footer class="modal-card-foot">
      <!-- <b-field horizontal> -->
      <p class="buttons">
        <BButton
          type="is-danger"
          @click="cancel"
        >
          Cancel
        </BButton>
        <BButton
          type="is-primary"
          @click="publishArticle"
        >
          Publish
        </BButton>
      </p>
      <!-- </b-field> -->
    </footer>
  </div>
</template>

<script>
import MarkdownEditor from './MarkdownEditor'

import Api from '@/api.js'
import { EventBus, EVENT_RELOAD_ARTICLE, EVENT_RELOAD_ARTICLE_LIST, EVENT_MARKDOWN_EDITOR_CONTENT_READY, EVENT_ARTICLE_EDITOR_CLOSED } from '@/event-bus.js';

export default {
  name: 'ArticleEditor',
  components: {
    MarkdownEditor
  },
  props: {
    articleId: {
      type: String,
      default: ''
    },
    isCreateNew: Boolean,
  },
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
    };
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
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`);
      let article = rsp.data;
      this.title = article.title
      this.content = article.content
      EventBus.$emit(EVENT_MARKDOWN_EDITOR_CONTENT_READY, this.content)
      this.tags = this.genTagObjectArray(article.tags, article.tags_id)
      this.oldTags = this.genTagObjectArray(article.tags, article.tags_id)
    },
    async getTags() {
      let api = new Api()
      let rsp = await api.getTags()
      this.$getLog().debug(`get tags: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        this.$getLog().error(`get tags failed: ${rsp.detail}`)
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
        this.$getUi().toast.fail('Title can not be empty!')
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
      this.$getLog().debug(`existTags: ${existTags}`)
      this.$getLog().debug(`newTags: ${newTags}`)
      let api = new Api()
      var rsp;
      let content = this.$refs.editor.content()
      if (this.isCreateNew) {
        rsp = await api.createArticle(this.title, content, existTags, newTags, this.publish)
      } else {
        let classifiedTags = this.classifyTags()
        rsp = await api.editArticle(this.articleId, this.title, content, classifiedTags.newChoiceAlreadyExistTags, classifiedTags.deselectedTags, classifiedTags.newTags)
      }
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`Failed to ${this.isCreateNew ? "create" : "update"} article: ${rsp.detail}`)
        return
      }

      this.$getUi().toast.success('Article is successfully published!')

      // reload article
      EventBus.$emit(EVENT_RELOAD_ARTICLE)
      // reload article list
      EventBus.$emit(EVENT_RELOAD_ARTICLE_LIST)

      // notify other markdown editor to reset content
      EventBus.$emit(EVENT_ARTICLE_EDITOR_CLOSED)
      this.$parent.close()
    },
    cancel() {
      // notify other markdown editor to reset content
      EventBus.$emit(EVENT_ARTICLE_EDITOR_CLOSED)
      this.$parent.close()
    },
    classifyTags() {
      return {
        newTags: this.tags.filter(t => {
          // if a tag is string typed, it must be a new tag
          return typeof (t) === 'string'
        }),
        newChoiceAlreadyExistTags: this.tags.filter(t => {
          // exist tags is an object
          if (typeof (t) === 'string') {
            return false
          }

          // oldTags is empty, every current tags(object typed) is exist tags
          if (this.oldTags.length === 0) {
            return true
          }

          // oldTags NOT contains any current tags
          return !this.oldTags.some(ot => {
            return ot.id === t.id
          })
        }).map(t => {
          // convert uuid array
          return t.id
        }),
        deselectedTags: this.deselectedTags.map(t => {
          // convert uuid array
          return t.id
        }),
      }
    },
    // generate tag object from tags and tags_id arrays
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
        // if the tag is string typed, it must be a new tag
        return
      }
      // avoid adding exist tag object to deselect tag array
      if (this.deselectedTags.filter(t => {
        return t.id === value.id
      }).length <= 0) {
        this.deselectedTags.push(value)
      }
    },
    addNewTag(value) {
      let isStringVal = (typeof (value) === 'string')
      let lowerCaseVal = isStringVal ? value.toLowerCase() : value.tag.toLowerCase()
      if (this.availableTags.some(t => {
        return t.tag.toLowerCase() === lowerCaseVal
      })) {
        this.tags = this.tags.filter(t => {
          if (isStringVal) {
            return t.toLowerCase() !== lowerCaseVal
          }
          return true
        })
      }
    }
  },
}
</script>