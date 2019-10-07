<template>
  <div>
    <div
      class="article-list"
      v-for="article in articles"
      v-bind:key="article.id"
    >
      <router-link :to="`/article/${article.id}`">
        <!-- title -->
        <div
          align="left"
          class="article-title"
        >{{article.title}}</div>

        <!-- summary -->
        <div
          align="left"
          class="article-summary"
        >
          {{article.summary}}
        </div>
      </router-link>

      <!-- bottom -->
      <section class="article-bottom columns">
        <div
          align="left"
          class="column is-1"
        >
          <span class="article-time ">
            {{formatDate(article.modify_time)}}
          </span>
        </div>
        <div
          align="left"
          class="column is-4"
          v-if="article.tags.length > 0"
        >
          <b-taglist
            class="article-tags"
            v-for="t in article.tags"
            v-bind:key="t"
          >
            <b-tag
              class="article-tag"
              v-if="article.tags.length > 0"
            >{{t}}</b-tag>
          </b-taglist>
        </div>
      </section>
      <hr class="short" />

      <br /><br />

    </div>
    <!-- pagination -->
    <b-pagination
      :total="articleCount"
      :current.sync="currentPage"
      range-before=3
      range-after=1
      order="is-centered"
      :rounded=true
      :per-page="pageSize"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
    >
    </b-pagination>
    <br />
  </div>
</template>
<script>
import Api from "@/api.js"
let dateFormat = require('dateformat')

export default {
  name: "ArticleList",
  components: {},
  props: {},
  data() {
    return {
      articleCount: 0,
      articles: [],
      currentPage: 1,
      pageSize: 10,
    };
  },
  async mounted() {
    await this.getArticleNumber();
    await this.getArticles();
  },
  methods: {
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

/* .tags {
  list-style: none;
  margin: 0;
  overflow: hidden;
  padding: 0;
}

.tags li {
  float: left;
}

.tag {
  background: #eee;
  border-radius: 3px 0 0 3px;
  color: #999;
  display: inline-block;
  height: 26px;
  line-height: 26px;
  padding: 0 20px 0 23px;
  position: relative;
  margin: 0 10px 10px 0;
  text-decoration: none;
  -webkit-transition: color 0.2s;
}

.tag::before {
  background: #fff;
  border-radius: 10px;
  box-shadow: inset 0 1px rgba(0, 0, 0, 0.25);
  content: "";
  height: 6px;
  left: 10px;
  position: absolute;
  width: 6px;
  top: 10px;
}

.tag::after {
  background: #fff;
  border-bottom: 13px solid transparent;
  border-left: 10px solid #eee;
  border-top: 13px solid transparent;
  content: "";
  position: absolute;
  right: 0;
  top: 0;
}

.tag:hover {
  background-color: crimson;
  color: white;
}

.tag:hover::after {
  border-left-color: crimson;
} */
</style>