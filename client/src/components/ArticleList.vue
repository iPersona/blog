<template>
  <div>
    <div
      v-for="article in articles"
      :key="article.id"
      class="article-list"
    >
      <RouterLink :to="`/article/${article.id}`">
        <!-- title -->
        <div
          align="left"
          class="article-title"
        >
          {{ article.title }}
        </div>

        <!-- summary -->
        <div
          align="left"
          class="article-summary"
        >
          {{ article.raw_content }}
        </div>
      </RouterLink>

      <!-- bottom -->
      <section class="article-bottom columns">
        <div
          align="left"
          class="column is-1"
        >
          <span class="article-time ">
            {{ formatDate(article.modify_time) }}
          </span>
        </div>
        <div
          v-if="article.tags.length > 0"
          align="left"
          class="column is-4"
        >
          <BTaglist
            v-for="t in article.tags"
            :key="t"
            class="article-tags"
          >
            <BTag
              v-if="article.tags.length > 0"
              class="article-tag"
            >
              {{ t }}
            </BTag>
          </BTaglist>
        </div>
      </section>
      <hr class="short">

      <br><br>
    </div>
    <!-- pagination -->
    <BPagination
      :total="articleCount"
      :current.sync="currentPage"
      range-before="3"
      range-after="1"
      order="is-centered"
      :rounded="true"
      :per-page="pageSize"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
    />
    <br>
  </div>
</template>
<script>
import Api from "@/api.js"
import Ui from "./utils/ui"

import { EventBus, EVENT_RELOAD_ARTICLE_LIST } from '@/event-bus'

import { mapGetters } from 'vuex'
import { TAG } from '@/store/modules/module-names'
import { TAG_ID, TAG_NAME } from '@/store/modules/store-types'

let dateFormat = require('dateformat')

export default {
  name: "ArticleList",
  components: {},
  props: {
    // tagId: {
    //   type: String,
    //   default: ''
    // },
    isFilteredByTag: {
      type: Boolean,
      default: false
    },
  },
  data() {
    return {
      articleCount: 0,
      articles: [],
      currentPage: 1,
      pageSize: 10,
      ui: new Ui(this),
    };
  },
  computed: {
    ...mapGetters(TAG, {
      tagId: TAG_ID,
      tagName: TAG_NAME,
    })
  },
  async mounted() {
    console.debug(`ArticleList mounted!`)
    await this.listenEventBus()
    await this.reloadArticle()
  },
  methods: {
    listenEventBus() {
      const self = this;
      EventBus.$on(EVENT_RELOAD_ARTICLE_LIST, async function () {
        console.log(`event-bus: ${EVENT_RELOAD_ARTICLE_LIST}`)
        await self.reloadArticle()
      })
    },
    async reloadArticle() {
      this.$log.debug(`reloadArticle...`)
      this.$log.debug(`isFilteredByTag: ${this.isFilteredByTag}, tagId: ${this.tagId}`)
      if (this.isFilteredByTag) {
        if (this.tagId !== undefined) {
          await this.getArticleNumberByTag()
          await this.getArticlesByTag()
        }
      } else {
        await this.getArticleNumber()
        await this.getArticles()
      }
    },
    async getArticlesByTag() {
      let api = new Api();
      let rsp = await api.getArticlesByTag(this.tagId, this.pageSize, (this.currentPage - 1) * this.pageSize);
      if (!Api.isSuccessResponse(rsp)) {
        this.$log.error(`failed to get articles by tag-${this.tagId}: ${rsp.detail}`);
        return;
      }
      this.articles = rsp.data;
      this.$log.debug(`rsp: ${JSON.stringify(rsp)}`);
    },
    async getArticleNumberByTag() {
      let api = new Api();
      let rsp = await api.getArticleNumberByTag(this.tagId);
      if (!Api.isSuccessResponse(rsp)) {
        this.$log.error(`failed to get article number by tag-${this.tagId}: ${rsp.detail}`);
        return;
      }
      if (!Api.isSuccessResponse(rsp)) {
        this.ui.toastFail(`failed to load article list: ${rsp.detail}`)
        return
      }
      this.$log.debug(`rsp: ${JSON.stringify(rsp)}`)
      this.articleCount = rsp.data;
      this.$log.debug(`article count: ${this.articleCount}`);
    },
    async getArticles() {
      let api = new Api();
      let rsp = await api.visitorViewAll(this.pageSize, (this.currentPage - 1) * this.pageSize);
      if (!Api.isSuccessResponse(rsp)) {
        this.$log.error(`failed to get articles: ${rsp.detail}`);
        return;
      }
      this.articles = rsp.data;
      this.$log.debug(`rsp: ${JSON.stringify(rsp)}`);
    },
    async getArticleNumber() {
      let api = new Api();
      let rsp = await api.getArticleNumber();
      if (!Api.isSuccessResponse(rsp)) {
        this.$log.error(`failed to get article number: ${rsp.detail}`);
        return;
      }
      this.articleCount = rsp.data;
      this.$log.debug(`article count: ${this.articleCount}`);
    },
    formatDate(datetime) {
      let d = Date.parse(datetime)
      return dateFormat(d, "yyyy-mm-dd")
    }
  }
};
</script>

<style scoped>
hr.short {
  align-content: left;
  width: 20%;
}

.article-list {
  align-content: left;
}

.article-title {
  font-weight: bold;
  font-size: xx-large;
}

.article-summary {
  font-size: large;
  color: gray;
}

.article-bottom {
  margin-top: 20px;
}

.article-time {
  font-size: small;
  color: gray;
  vertical-align: middle;
  text-align: center;
}

.article-tags {
  font-size: small;
  display: inline;
}

.article-tag {
  margin-right: 10px;
}

.read-more {
  /* padding: 12px; */
  vertical-align: middle;
}

.content {
  background-color: hsl(200, 33%, 58%);
  color: white;
  text-decoration: none;
  text-transform: uppercase;
  background: hsl(200, 33%, 58%);
  float: left;
  font-size: small;
  height: 15px;
}
</style>