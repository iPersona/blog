<template>
  <div
    class="editor"
    align="left"
  >
    <textarea id="editor" />
    <!-- 可能是vscode格式化插件的问题，div老是被缩进4个空格，不能和起始div对齐 -->
    <!-- eslint-disable-next-line -->
    </div>
</template>

<script>
import EasyMDE from 'easymde'
import "easymde/dist/easymde.min.css"

import Hljs from 'highlight.js'
import 'highlight.js/styles/solarized-dark.css'

import { EventBus, EVENT_MARKDOWN_EDITOR_CONTENT_READY } from '@/event-bus.js'

// import { EventBus, EVENT_HIDE_HEADER, EVENT_SHOW_HEADER } from '@/event-bus.js'

export default {
  name: 'MarkdownEditor',
  component: {},
  props: {
    placeholder: {
      type: String,
      default: ''
    },
    minHeight: {
      type: Number,
      default: 500
    },
    toolbarStyle: {
      type: String,
      default: 'Article'
    },
    initialValue: {
      type: String,
      default: ''
    },
  },
  data() {
    return {
      editor: null,
      toolbarSet: {
        Article: ['heading', 'bold', 'italic', 'strikethrough', '|', 'quote', 'unordered-list', 'ordered-list', 'link', 'image', 'code', {
          name: "table",
          action: EasyMDE.drawTable,
          className: "fa fa-table",
          title: "Table",
        }, 'horizontal-rule', 'preview', 'side-by-side', 'fullscreen'],

        Comment: [
          "heading", "bold", "italic", "|", "quote", "code", "link", "|", "unordered-list", "ordered-list", "|", "preview"
        ]
      }
    }
  },
  mounted() {
    this.initEditor()
    this.listenEvent()
  },
  methods: {
    listenEvent() {
      const self = this;
      EventBus.$on(EVENT_MARKDOWN_EDITOR_CONTENT_READY, async function (val) {
        console.log(`event-bus: ${EVENT_MARKDOWN_EDITOR_CONTENT_READY}`)
        self.editor.value(val)
      })
    },
    initEditor() {
      this.editor = new EasyMDE({
        element: document.getElementById('editor'),
        minHeight: `${this.minHeight}px`,
        placeholder: this.placeholder,
        status: false,
        toolbar: this.toolbarSet[this.toolbarStyle],
        autofocus: false,
        initialValue: this.initialValue,
        uploadImage: true,
        // onToggleFullScreen: this.onToggleFullScreen,
        renderingConfig: {
          codeSyntaxHighlighting: true,
          hljs: Hljs,
        },
        spellChecker: false,
      })
    },

    // onToggleFullScreen(isFullScreen) {
    //   console.debug(`full-screen: ${isFullScreen}`)
    //   if (isFullScreen) {
    //     EventBus.$emit(EVENT_HIDE_HEADER)
    //   } else {
    //     EventBus.$emit(EVENT_SHOW_HEADER)
    //   }
    // },

    content() {
      return this.editor.value()
    },

    setContent(content) {
      console.log(`set-content: ${content}`)
      this.editor.value(content)
    }
  },
}
</script>

<style scoped>
/* this editor style is use to recover the EasyMDE style impacted by Buefy */
.editor >>> ul {
  margin: 5px;
  padding: 5px;
  list-style: initial;
}

.editor >>> ol {
  margin: 5px;
  padding: 5px;
  list-style-type: decimal;
}

.editor >>> .editor-preview pre {
  padding: 10px;
}

.editor >>> .editor-toolbar button {
  margin-bottom: 0;
}

.editor >>> .editor-preview code {
  border-radius: 5px;
}

.editor >>> .editor-toolbar button i {
  font-size: initial;
}
</style>