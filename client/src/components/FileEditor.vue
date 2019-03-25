<template>
  <div>
    <!-- 按键区 -->
    <div class="d-flex align-items-stretch">
      <b-btn
        @click="validate"
        variant="success"
        class="m-1"
      >
        JSON 校验
      </b-btn>
      <b-btn
        @click="save"
        variant="primary"
        class="m-1"
        :disabled="!enableSave"
      >
        保存
      </b-btn>
      <b-btn
        @click="saveFile"
        variant="primary"
        class="m-1"
      >
        拉取
      </b-btn>
    </div>
    <hr />
    <!-- 信息提示区 -->
    <b-alert
      :show="dismissCountDown"
      dismissible
      fade
      :variant="variant"
      @dismissed="dismissCountDown=0"
      @dismiss-count-down="countDownChanged"
    >
      {{ checkerInfo}} {{dismissCountDown}}s ...
    </b-alert>

    <!-- 文本编辑区 -->
    <!-- <j-s-o-n-editor
      :onChange="onChange"
      :json="text"
      :options="opts"
    /> -->
    <!-- <b-card
      :header="fileName"
      header-tag="header"
    > -->
    <b-card header-tag="header">
      <h6
        slot="header"
        class="mb-0"
      >{{fileName}}
        <b-badge
          variant="primary"
          disabled="modifyIdentifier.length <= 0"
        >{{modifyIdentifier}}</b-badge>
      </h6>
      <!-- [自动适配宽度与高度](https://github.com/securingsincity/react-ace/issues/415#issuecomment-404946384) -->
      <editor
        v-model="contentText"
        @init="initEditor"
        lang="json"
        theme="github"
        width="inherit"
        height="inherit"
        :options="options"
      >
        ></editor>
    </b-card>
  </div>
</template>

<script>

  // import JSONEditor from 'vue2-jsoneditor'
  import Api from '@/api.js'
  import FileSaver from 'file-saver'

  export default {
    name: 'FileEditor',
    components: {
      // JSONEditor
      editor: require('vue2-ace-editor'),
    },
    props: {
      fileName: String,
      path: String
    },
    data() {
      return {
        opts: {
          modes: ["tree", "code"],
        },
        options: {
          minLines: 1,
          maxLines: Infinity
        },

        variant: "success",
        checkerInfo: "",
        dismissSecs: 5,
        dismissCountDown: 0,
        contentText: '',
        contentTextBackup: '',
        modifyIdentifier: '',
        isFirstLoad: true,
        enableSave: false,
      }
    },
    mounted() {

    },
    destroyed() {
      console.log('FileEditor-destroyed');
    },
    watch: {
      path: async function (newVal, oldVal) {
        this.modifyIdentifier = ''; // 首次加载不添加文件修改标识符
        this.isFirstLoad = true;
        this.enableSave = false;
        console.log("path changed (" + oldVal + " -> " + newVal + ")");
        this.contentText = await this.getText(newVal)
        this.contentTextBackup = this.contentText;
      },

      contentText: function (newVal, oldVal) {
        // 加载后不需要提示文件修改标识符
        if (this.isFirstLoad) {
          this.isFirstLoad = false;
          this.enableSave = false;
          return;
        }

        if (newVal === this.contentTextBackup) {
          this.modifyIdentifier = '';
          this.enableSave = false;
          return;
        }

        this.modifyIdentifier = '修改';
        this.enableSave = true;

        // 通知父组件文件状态已改变
        var args = {
          isModified: true
        }
        this.$emit('fileModified', args);
      },
    },
    methods: {
      async getText(path) {
        console.log("getText() invoked!");
        if (path === '') {
          return ''
        }

        console.log("path2222: " + path);
        let api = new Api();
        let content = await api.getFile(path);
        if (typeof content === 'object') {
          // 使用易读模式打印
          content = JSON.stringify(content, null, 2);
        }
        console.log("context: " + content);
        return content;
      },
      // onChange(newJson) {
      //   console.log("onchange");
      //   this.text = JSON.stringify(newJson);
      // },
      validate() {
        try {
          console.log("text: " + JSON.stringify(this.contentText));
          let jsonObj = JSON.parse(this.contentText);
          this.showMsg("This is a valid JSON");
          return jsonObj;
        } catch (e) {
          let msg = e.message;
          console.log("invalid json string: " + msg);
          this.showError(msg);
          return this.contentText;
        }
      },
      showError(msg) {
        this.dismissCountDown = 10
        this.checkerInfo = msg;
        this.variant = "danger";
      },
      showMsg(msg) {
        this.dismissCountDown = 3
        this.checkerInfo = msg;
        this.variant = "success";
      },
      countDownChanged(dismissCountDown) {
        this.dismissCountDown = dismissCountDown
      },
      initEditor: function (editor) {
        require('brace/mode/json');
        require('brace/theme/github');
      },
      async save() {
        let api = new Api();
        try {
          let rsp = await api.saveFile(this.path, this.contentText);
          console.log("rsp: " + rsp);
          this.showMsg("File is successfully saved!");

          // 保存成功后需要恢复修改状态
          this.isFirstLoad = true;
          this.enableSave = false;
          this.modifyIdentifier = ''; // 去除文件修改标识符
          this.contentTextBackup = this.contentText;  // 重新设置初始文本
          // 通知父组件，文本未被修改
          var args = {
            isModified: false
          }
          this.$emit('fileModified', args);
        } catch (error) {
          console.log("rsp-err: " + error.message);
          this.showError("Save failed: " + error.message);
        }
      },
      saveFile() {
        var blob = new Blob([this.contentText], { type: "text/plain;charset=utf-8" });
        FileSaver.saveAs(blob, this.fileName);
      }
    }
  }
</script>

