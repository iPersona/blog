<template>
  <div>
    <div
      v-for="article in articles"
      :key="article.id"
      class="article-list"
    >
      <RouterLink :to="{name: 'article', params: {id: article.id}}">
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
      <section class="article-bottom">
        <b-field
          grouped
          align="left"
        >
          <span class="article-time">
            {{ formatDate(article.modify_time) }}
          </span>

          <div
            v-if="article.tags.length > 0"
            align="left"
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
        </b-field>
      </section>
      <br><br>
    </div>

    <!-- article loading component -->
    <InfiniteLoading
      spinner="waveDots"
      @infinite="infiniteHandler"
    >
      <div slot="no-more">
        No more articles
      </div>
    </InfiniteLoading>

    <!-- back to top button -->
    <BackToTop
      bottom="50px"
      right="50px"
    >
      <chevron-up-icon size="3x" />
    </BackToTop>
  </div>
</template>
<script>
import InfiniteLoading from 'vue-infinite-loading'
import BackToTop from 'vue-backtotop'
import Api from "@/api.js"
import { ChevronUpIcon } from 'vue-feather-icons'

import { EventBus, EVENT_RELOAD_ARTICLE_LIST } from '@/event-bus'

import { mapGetters } from 'vuex'
import { TAG } from '@/store/modules/module-names'
import { TAG_ID, TAG_NAME } from '@/store/modules/store-types'

let dateFormat = require('dateformat')

export default {
  name: "ArticleList",
  components: {
    InfiniteLoading,
    BackToTop,
    ChevronUpIcon
  },
  props: {
    isFilteredByTag: {
      type: Boolean,
      default: false
    },
  },
  data() {
    return {
      // articleCount: 0,
      articles: [],
      currentPage: 1,
      pageSize: 10,
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
    // await this.reloadArticle()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_RELOAD_ARTICLE_LIST)
  },
  methods: {
    listenEventBus() {
      const self = this;
      EventBus.$on(EVENT_RELOAD_ARTICLE_LIST, async function () {
        console.log(`event-bus: ${EVENT_RELOAD_ARTICLE_LIST}`)
        // Reset current page to load most recent articles
        self.currentPage = 1;
        await self.reloadArticle()
      })
    },
    async reloadArticle($state) {
      this.$getLog().debug(`reloadArticle...`)
      this.$getLog().debug(`isFilteredByTag: ${this.isFilteredByTag}, tagId: ${this.tagId}`)
      if (this.isFilteredByTag) {
        if (this.tagId !== undefined) {
          await this.getArticlesByTag($state)
        }
      } else {
        await this.getArticles($state)
      }
    },
    async getArticlesByTag($state) {
      let api = new Api();
      let rsp = await api.getArticlesByTag(this.tagId, this.pageSize, (this.currentPage - 1) * this.pageSize);
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`);
      if (!Api.isSuccessResponse(rsp)) {
        this.$getLog().error(`failed to get articles by tag-${this.tagId}: ${rsp.detail}`);
        return;
      }

      if ($state === undefined) {
        // Not from infinite-scroll event, reset articles to data directly
        this.articles = rsp.data
        return
      }

      if (rsp.data.length > 0) {
        console.log(`$state-load`)
        this.currentPage += 1;
        this.articles.push(...rsp.data)
        $state.loaded()
      } else {
        console.log(`$state-complete`)
        $state.complete()
      }
    },
    async getArticles($state) {
      let api = new Api();
      let rsp = await api.visitorViewAll(this.pageSize, (this.currentPage - 1) * this.pageSize);
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`);
      if (!Api.isSuccessResponse(rsp)) {
        this.$getLog().error(`failed to get articles: ${rsp.detail}`);
        return;
      }

      if ($state === undefined) {
        // Not from infinite-scroll event, reset articles to data directly
        this.articles = rsp.data
        return
      }

      if (rsp.data.length > 0) {
        this.currentPage += 1;
        this.articles.push(...rsp.data)
        console.log(`$state-load`)
        $state.loaded()
      } else {
        console.log(`$state-complete`)
        $state.complete()
      }
    },
    formatDate(datetime) {
      let d = Date.parse(datetime)
      return dateFormat(d, "yyyy-mm-dd")
    },
    async infiniteHandler($state) {
      console.log(`infiniteHandler.state: ${JSON.stringify($state)}`)
      this.reloadArticle($state)
    }
  }
};
</script>

<style scoped>
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
  font-style: italic;
  color: gray;
  display: flex;
  align-items: center;
  margin-right: 30px;
}

.article-tags {
  font-size: small;
  display: inline;
}

.article-tag {
  margin-right: 10px;
}

.tags:not(:last-child) {
  margin-bottom: 0;
}

.tags .tag {
  margin-bottom: 0;
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