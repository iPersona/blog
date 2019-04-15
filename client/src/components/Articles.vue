<template>
  <div>
    <div
      v-for="v in articles"
      v-bind:key="getCaseKey(v)"
      v-bind:active="tab === getCaseKey(v)"
      @click="selectItem(v)"
    ></div>
  </div>
</template>
<script>
import Api from "@/api.js";
import Utils from "@/utils.js";

export default {
  name: "Articles",
  components: {},
  props: {},
  data() {
    return {
      articles: [],
      tab: "",
      page: 0
    };
  },
  async mounted() {
    this.getArticleSlice(10, this.page * 10);
  },
  methods: {
    async getArticleSlice(limit, offset) {
      let api = new Api();
      let rsp = await api.visitorViewAll(limit, offset);
      // let rsp = await api.adminView("fc2e4b7c-664c-40e4-99f2-cd5c6178c722");
      // let rsp = await api.login("admin", "admin", true);
      // this.articles = rsp.data;
      // let rsp = await api.createArticle(
      //   "title",
      //   "# markdown blog\n- a\n-b",
      //   undefined,
      //   ["demo"]
      // );
      console.log("rsp: " + JSON.stringify(rsp));
    },

    getCaseKey(caseObj) {
      return "cid-" + Utils.getObjectHash(caseObj);
    },

    selectItem(v) {
      this.tab = this.getCaseKey(v);
    }
  }
};
</script>

