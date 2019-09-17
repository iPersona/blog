<template>
  <div></div>
</template>

<script>
import Api from "@/api.js";
export default {
  name: "Home",
  components: {},
  data() {
    return {};
  },
  async mounted() {
    // let api = new Api();
    // let rsp = await api.login("admin", "password", true);
    // let rsp = await api.createArticle(
    //   "title",
    //   "# markdown blog\n- a\n-b",
    //   undefined,
    //   ["demo"]
    // );
    // let rsp = await api.deleteArticle("56243c98-a365-4c44-a40e-f1cae3c998ba");
    // let rsp = await api.adminView("56243c98-a365-4c44-a40e-f1cae3c998ba");
    // let rsp = await api.adminViewRawArticle("ec7224e1-217a-45cd-9bf5-8a526ce4b9dc");
    // let rsp = await api.adminViewAll(5, 0);
    // let rsp = await api.editArticle("57567264-f216-4736-9305-66aaafe17705", "new title", "# new raw content\n- 1\n-2", undefined, undefined, ["a", "b"]);
    // let rsp = await api.publishArticle("57567264-f216-4736-9305-66aaafe17705");
    // console.log("rsp: " + JSON.stringify(rsp));
  },
  methods: {
    handleSelect(key, keyPath) {
      console.log(key, keyPath);
    }
  }
};
</script>

