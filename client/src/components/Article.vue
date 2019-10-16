<template>
  <div>
    <section
      v-if="isAdmin"
      align="right"
      class="container"
    >
      <div>
        <BButton
          icon-pack="fas"
          icon-left="edit"
          @click="editArticle"
        >
          Edit
        </BButton>
      </div>
    </section>
    <section class="container">
      <div class="container">
        <h1 class="title">
          {{ article.title }}
        </h1>
      </div>
    </section>
    <!-- article content -->
    <section
      class="container"
      align="left"
    >
      <div v-highlight>
        <!-- v-html="compiledMarkdown" -->
        {{ compiledMarkdown }}
      </div>
    </section>
    <br><br>
    <section
      class="container"
      align="left"
    >
      <b>Tags: </b>
      <BTaglist
        v-for="t in article.tags"
        :key="t"
        class="article-tags"
      >
        <BTag
          v-if="hasTags()"
          class="article-tag"
        >
          {{ t }}
        </BTag>
      </BTaglist>
    </section>

    <BModal
      :active.sync="isEditArticle"
      has-modal-card
      full-screen
      :can-cancel="false"
    >
      <ArticleEditor
        :article-id="this.$route.params.id"
        :is-create-new="false"
      />
    </BModal>
  </div>
</template>

<script>
import Api from "@/api.js";
import Ui from './utils/ui.js'
import Log from './utils/log.js'
import marked from 'marked'
import hljs from 'highlight.js'
import 'highlight.js/styles/monokai-sublime.css'
import { mapGetters } from 'vuex'
import { IS_ADMIN } from '@/store/modules/store-types.js'
import { USER } from '@/store/modules/module-names'
import ArticleEditor from './ArticleEditor'
import { EventBus, EVENT_RELOAD_ARTICLE } from '@/event-bus.js'

export default {
  name: "Article",
  components: {
    ArticleEditor,
  },
  props: {},
  data() {
    return {
      ui: new Ui(this),
      log: new Log(this),
      article: {},
      isEditArticle: false,
    };
  },
  computed: {
    compiledMarkdown: function () {
      this.log.debug(`compiledMarkdown: ${this.article.content}`);
      if (this.article.content === undefined) {
        return marked('')
      }
      return marked(this.article.content);
    },
    ...mapGetters(USER, {
      isAdmin: IS_ADMIN,
    }),
  },
  async mounted() {
    await this.getArticle()
    await this.listenEventBus()
  },
  methods: {
    listenEventBus() {
      const self = this;
      EventBus.$on(EVENT_RELOAD_ARTICLE, async function () {
        console.log(`event-bus: reload-data`)
        await self.getArticle()
      })
    },
    async getArticle() {
      let api = new Api();
      let id = this.$route.params.id;
      let rsp = await api.visitorViewArticle(id);
      if (!Api.isSuccessResponse(rsp)) {
        return;
      }
      this.log.debug(`rsp: ${JSON.stringify(rsp)}`);
      this.article = rsp.data;
      this.trimTags()
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
    },
    editArticle() {
      this.isEditArticle = true
    },
    hasTags() {
      return this.article.tags !== undefined
        && this.article.tags.length > 0
    },
    trimTags() {
      this.article.tags = this.article.tags.filter(t => {
        return t !== null
      })
    }
  }
};
</script>
<style scoped>
.article-tags {
  font-size: small;
  display: inline;
}

.article-tag {
  margin-right: 10px;
}
</style>