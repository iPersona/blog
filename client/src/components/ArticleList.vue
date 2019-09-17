<template>
  <div>
    <!-- Article list -->
    <section>
      <div
        class="title"
        v-for="article in articles"
        v-bind:key="article.id"
      >
        <router-link :to="`/article/${article.id}`">{{article.title}}</router-link>
      </div>
    </section>

    <br /><br />
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
import Api from "@/api.js";

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
    }
  }
};
</script>

