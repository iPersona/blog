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
  },
  methods: {
    handleSelect(key, keyPath) {
      console.log(key, keyPath);
    }
  }
};
</script>

