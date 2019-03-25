<template>
  <div class="container-fluid">
    <!-- <b-alert
      show
      variant="secondary"
    >
      <h2>文件编辑器</h2>
    </b-alert> -->
    <div class="row">
      <div class="col">
        <file-browser @selectedNodeChanged="selectedNodeChanged" />
      </div>
      <div class="col-9">
        <file-editor
          ref="editor"
          :path="path"
          :fileName="fileName"
          @fileModified="fileModified"
        />
      </div>
    </div>

    <!-- 保存确认对话框 -->
    <b-modal
      ref="saveModal"
      centered
      title="文件已被修改但尚未保存"
      ok-title="保存"
      cancel-title="丢弃"
      @ok="saveFile"
      @cancel="discardChanges"
    >
      <p class="my-4">是否保存文件？</p>
    </b-modal>
  </div>
</template>

<script>
  import FileBrowser from './FileBrowser'
  import FileEditor from './FileEditor'

  export default {
    name: 'CaseEditor',
    components: {
      FileBrowser,
      FileEditor,
    },
    data() {
      return {
        path: '',
        fileName: 'Please selected a file',
        isFileModified: false,
        currentPath: '',
        currentFileName: '',
      }
    },
    mounted() {

    },
    methods: {
      async selectedNodeChanged(args) {
        console.log("args: " + JSON.stringify(args));

        // 记录跳转信息
        this.currentPath = args.path;
        this.currentFileName = args.currentFileName;

        // TODO: 若文件已修改但未保存，则提示，同时不切换文本内容
        if (this.isFileModified) {
          this.$refs.saveModal.show();
          return;
        }

        this.fileName = args.path;
        this.path = args.path;
        this.isFileModified = false;
      },
      fileModified(args) {
        this.isFileModified = args.isModified;
      },
      saveFile() {
        // 保存文件
        this.$refs.editor.save();
        this.redirectToSelectedNode();  // 重定向到选中的节点文件上
      },
      discardChanges() {
        // 丢弃修改
        this.isFileModified = false;
        this.redirectToSelectedNode();  // 重定向到选中的节点文件上
      },
      redirectToSelectedNode() {
        this.path = this.currentPath;
        this.fileName = this.currentFileName;
      }
    }
  }
</script>

