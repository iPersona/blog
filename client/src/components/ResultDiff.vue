<template>
  <div class="py-0 my-0">
    <!-- 去除上边框间隙 -->
    <vue-code-diff
      class="py-0 my-0"
      name
      :old-string="exp"
      :new-string="act"
      :outputFormat="outputFormat"
      :context="5"
    />
  </div>
</template>

<script>
  import VueCodeDiff from 'vue-code-diff'
  export default {
    name: 'ResultDiff',
    props: {
      exp: String,
      act: String,
    },
    components: { VueCodeDiff },
    data() {
      return {
        outputFormat: 'side-by-side',
        context: 100
      }
    }
  }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
  h3 {
    margin: 40px 0 0;
  }
  ul {
    list-style-type: none;
    padding: 0;
  }
  li {
    display: inline-block;
    margin: 0 10px;
  }
  a {
    color: #42b983;
  }
</style>
