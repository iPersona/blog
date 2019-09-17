<template>
  <section>
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

    <b-field horizontal>
      <p class="buttons">
        <b-button
          type="is-primary"
          @click="publishArticle"
        >Publish</b-button>
        <b-button
          type="is-danger"
          @click="cancel"
        >Cancel</b-button>
      </p>
    </b-field>
  </section>

</template>

<script>
import 'tui-editor/dist/tui-editor.css'
import 'tui-editor/dist/tui-editor-contents.css'
import 'codemirror/lib/codemirror.css'
import { Editor } from '@toast-ui/vue-editor'

import Api from '@/api.js'
import Log from './utils/log.js'
import Ui from './utils/ui.js'

// import Editor from '@toast-ui/vue-editor'

export default {
  name: 'ArticleEditor',
  data() {
    return {
      title: '',
      content: '',
      previewStyle: 'vertical',
      tags: [], // final selected tag
      filteredTags: [], // tags show in autocomplete panel
      availableTags: [],
      publish: true,
      log: new Log(this),
      ui: new Ui(this),
    };
  },
  components: {
    'editor': Editor
  },
  mounted() {
    this.getTags()
  },
  methods: {
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
      let rsp = await api.createArticle(this.title, this.content, existTags, newTags, this.publish)
      if (!Api.isSuccessResponse(rsp)) {
        this.ui.toastFail(`Failed to create article: ${rsp.detail}`)
        return
      }

      this.ui.toastSuccess('Article is successfully published!')
      // go back to previous view
      this.$router.go(-1)
    },
    cancel() {

    }
  },
}
</script>