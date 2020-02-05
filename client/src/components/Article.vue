<template>
  <div>
    <nav
      v-if="isAdmin"
      class="container"
    >
      <b-field
        expanded
        position="is-right"
      >
        <p class="control">
          <BButton
            icon-pack="fas"
            icon-left="edit"
            @click="editArticle"
          >
            Edit
          </BButton>
        </p>

        <p class="control">
          <b-dropdown
            hoverable
            aria-role="list"
            expanded
          >
            <button
              slot="trigger"
              class="button"
            >
              <b-icon
                pack="fas"
                icon="ellipsis-h"
              />
            </button>

            <b-dropdown-item
              aria-role="listitem"
              @click="deleteArticle"
            >
              <b-icon
                pack="fas"
                icon="trash-alt"
              />
              <span>
                Delete
              </span>
            </b-dropdown-item>
          </b-dropdown>
        </p>
      </b-field>
    </nav>
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
      <!-- eslint-disable vue/no-v-html -->
      <div
        v-highlight
        align="left"
        v-html="compiledMarkdown"
      />
    </section>
    <br>
    <br>
    <section
      class="container"
      align="left"
      style="float: left;"
    >
      <span>
        <b>Tags: </b>
      </span>
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
    <br>

    <BModal
      :active.sync="isEditArticle"
      has-modal-card
      full-screen
      :can-cancel="false"
    >
      <ArticleEditor
        :article-id="articleId"
        :is-create-new="false"
      />
    </BModal>
    <hr>

    <!-- Comments -->
    <NewComment
      v-if="isLogin"
      id="commentEditor"
      :article-id="articleId"
    />
    <section class="container">
      <Comments :article-id="articleId" />
    </section>
  </div>
</template>

<script>
import Api from "@/api.js";
import marked from "marked";
import hljs from "highlight.js";
import "highlight.js/styles/monokai-sublime.css";
import { mapGetters } from "vuex";
import { IS_ADMIN, IS_LOGIN } from "@/store/modules/store-types.js";
import { USER } from "@/store/modules/module-names";
import ArticleEditor from "./ArticleEditor";
import Comments from "./Comments";
import { EventBus, EVENT_RELOAD_ARTICLE, EVENT_SCROLL_TO_COMMENT_EDITOR } from "@/event-bus.js";
import NewComment from "./NewComment";
import VueScrollTo from 'vue-scrollto'

export default {
  name: "Article",
  components: {
    ArticleEditor,
    Comments,
    NewComment
  },
  props: {},
  data() {
    return {
      articleId: this.$route.params.id,
      article: {},
      isEditArticle: false
    };
  },
  computed: {
    compiledMarkdown: function () {
      this.$getLog().debug(`compiledMarkdown: ${this.article.content}`);
      if (this.article.content === undefined) {
        return marked("");
      }
      return marked(this.article.content);
    },
    ...mapGetters(USER, {
      isAdmin: IS_ADMIN,
      isLogin: IS_LOGIN,
    })
  },
  async mounted() {
    await this.getArticle();
    await this.listenEventBus();
  },
  beforeDestroy() {
    EventBus.$off(EVENT_RELOAD_ARTICLE)
    EventBus.$off(EVENT_SCROLL_TO_COMMENT_EDITOR)
  },
  methods: {
    listenEventBus() {
      const self = this
      // reload article
      EventBus.$on(EVENT_RELOAD_ARTICLE, async function () {
        console.log(`event-bus: ${EVENT_RELOAD_ARTICLE}`)
        await self.getArticle()
      })
      // scroll to comment article
      EventBus.$on(EVENT_SCROLL_TO_COMMENT_EDITOR, function () {
        console.log(`event-bus: ${EVENT_RELOAD_ARTICLE}`)
        self.scrollToCommentEditor()
      })
    },
    async scrollToCommentEditor() {
      VueScrollTo.scrollTo('#commentEditor', 500)
    },
    async getArticle() {
      let api = new Api();
      let rsp = await api.visitorViewArticle(this.articleId);
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`failed to load article: ${rsp.detail}`);
        return;
      }
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`);
      this.article = rsp.data;
      this.trimTags();
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
      this.isEditArticle = true;
    },
    hasTags() {
      return this.article.tags !== undefined && this.article.tags.length > 0;
    },
    trimTags() {
      this.article.tags = this.article.tags.filter(t => {
        return t !== null;
      });
    },
    deleteArticle() {
      let self = this
      this.$buefy.dialog.confirm({
        title: 'Deleting article',
        message: 'Are you sure you want to <b>delete</b> this article? This action cannot be undone.',
        confirmText: 'Delete Article',
        type: 'is-danger',
        hasIcon: true,
        onConfirm: () => {
          self.doDeleteArticle()
        }
      })
    },
    async doDeleteArticle() {
      let api = new Api()
      let rsp = await api.deleteArticle(this.articleId)
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`failed to delete article: ${rsp.detail}`)
        return
      }
      this.$getUi().toast.success(`delete article successfully!`)
      // redirect to article list
      this.$router.replace({ name: 'articles' })
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