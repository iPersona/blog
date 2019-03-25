<template>
  <div>
    <b-btn
      block
      variant="outline-success"
      @click="editWithStackedit"
    >Edit</b-btn>
    <editor
      v-model="text"
      previewStyle="vertical"
    />
  </div>
</template>
<script>
  import 'tui-editor/dist/tui-editor.css';
  import 'tui-editor/dist/tui-editor-contents.css';
  import 'codemirror/lib/codemirror.css';
  import Editor from '@toast-ui/vue-editor/src/Editor.vue'


  export default {
    name: "Home",
    components: {
      'editor': Editor
    },
    data() {
      return {
        text: ''
      }
    },
    mounted() {
    },
    methods: {


    }
  }
</script>

