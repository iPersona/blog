<template>
  <div>
    <section class="container">
      <div class="container">
        <h1 class="title">{{article.title}}</h1>
      </div>
    </section>
    <section
      class="container"
      align="left"
    >
      <div
        v-highlight
        v-html="compiledMarkdown"
      ></div>
    </section>
  </div>
</template>

<script>
import Api from "@/api.js";
import Ui from './utils/ui.js'
import Log from './utils/log.js'
import marked from 'marked'
import hljs from 'highlight.js'
import 'highlight.js/styles/monokai-sublime.css'

export default {
  name: "Article",
  components: {
  },
  props: {},
  computed: {
    compiledMarkdown: function () {
      this.log.debug(`compiledMarkdown: ${this.article.content}`);
      if (this.article.content === undefined) {
        return marked('')
      }
      return marked(this.article.content);
    }
  },
  data() {
    return {
      ui: new Ui(this),
      log: new Log(this),
      article: {},
    };
  },
  async mounted() {
    this.getArticle();
  },
  methods: {
    async getArticle() {
      let api = new Api();
      let id = this.$route.params.id;
      let rsp = await api.visitorViewArticle(id);
      if (!Api.isSuccessResponse(rsp)) {
        return;
      }
      this.log.debug(`rsp: ${JSON.stringify(rsp)}`);
      let demoData = `# Marked - Markdown Parser
\`\`\` javascript
  let a = new Api();
  console.log('good');
\`\`\`
---
# Next subject
is good to see you 
`;
      this.article = rsp.data;
      this.article.content = demoData;
    },
    initMarked() {
      marked.setOptions({
        renderer: new marked.Renderer(),
        highlight: function (code, lang) {
          if (lang && hljs.getLanguage(lang)) {
            return hljs.highlight(lang, code, true).value;
          } else {
            return hljs.highlightAuto(code).value;
          }
        },
        // langPrefix: '',
        // highlight: function (code, lang) {
        //   return hljs.highlightAuto(code, [lang]).value
        // },
        gfm: true,
        tables: true,
        breaks: false,
        pedantic: false,
        sanitize: false,
        smartLists: true,
        smartypants: false,
        xhtml: false
      });
    }
  }
};
</script>