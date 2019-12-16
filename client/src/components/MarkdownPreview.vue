<template>
  <!-- eslint-disable vue/no-v-html -->
  <div
    v-highlight
    class="preview"
    align="left"
    v-html="compiledMarkdown"
  />
</template>

<script>
import marked from 'marked'
import hljs from 'highlight.js'
import 'highlight.js/styles/monokai-sublime.css'

export default {
  name: 'MarkdownPreview',
  components: {},
  props: {
    content: {
      type: String,
      default: ''
    }
  },
  data() {
    return {}
  },
  computed: {
    compiledMarkdown: function () {
      this.$getLog().debug(`compiledMarkdown: ${this.content}`);
      return marked(this.content);
    },
  },
  mounted() {

  },
  methods: {

  },
}
</script>

<style scoped>
.preview >>> ul {
  margin: 10px;
  padding: 10px;
  list-style: initial;
}

.preview >>> ol {
  margin: 10px;
  padding: 10px;
  list-style-type: decimal;
}

.preview >>> pre {
  padding: 0;
}

.preview >>> button {
  margin-bottom: 0;
}

.preview >>> code {
  border-radius: 5px;
}

.preview >>> .editor-toolbar button i {
  font-size: initial;
}
</style>