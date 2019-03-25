<template>
  <div>
    <hr />
    <div>
      <!-- 用例描述 -->
      <div>
        <h5 class="text-left">用例描述</h5>
        <p class="text-left">{{ description }}</p>
      </div>
    </div>

    <hr>
    <!-- 测试结果 -->
    <div>
      <h5 class="text-left">测试结果</h5>
      <!-- diff 视图 -->
      <div>
        <div class="row">
          <div class="col mr-0 pr-0">
            <b-btn
              block
              variant="outline-success"
            >
              预期
            </b-btn>
          </div>
          <div class="col ml-0 pl-0">
            <b-btn
              block
              variant="outline-danger"
            >
              实际
            </b-btn>
          </div>
        </div>
        <div class="row">
          <div class="col">
            <result-diff
              :exp="expData"
              :act="actData"
            ></result-diff>
          </div>
        </div>
      </div>
    </div>
    <!-- 日志显示 -->
    <div class="d-flex">
      <b-button-group>
        <b-btn
          variant="primary"
          @click="showLog"
        >查看日志</b-btn>
        <b-btn
          variant="primary"
          @click="showResult"
        >查看实际结果</b-btn>
      </b-button-group>
      <b-modal
        ref="logView"
        size="lg"
        centered
        :title="path"
        ok-only
      >
        <editor
          v-model="log"
          @init="initEditor"
          lang="json"
          theme="github"
          width="inherit"
          height="500"
        ></editor>
      </b-modal>
    </div>
  </div>
</template>
<script>
  import ResultDiff from "./ResultDiff"
  import Api from "@/api.js"
  import Utils from "@/utils.js"

  export default {
    name: "TaskInfo",
    components: {
      ResultDiff,
      editor: require('vue2-ace-editor'),
    },
    data() {
      return {
        description: '',
        expData: '{}',
        actData: '{}',
        log: '',
        options: {
          minLines: 1,
          maxLines: Infinity
        },
        path: '',
      }
    },
    mounted() { },
    methods: {
      async updateData(args) {
        try {
          // 用于获取log文件路径
          this.path = Utils.getLog(args.caseSet, args.caseId);
          // 加载用例信息
          let api = new Api();
          let rsp = await api.getTaskInfo(args.caseSet, args.caseId);
          console.log("rsp: " + JSON.stringify(rsp));
          this.description = rsp.description;
          this.expData = Utils.prettyStringify(rsp.expect);
          // 未执行的用例不会有 `result` 字段
          this.actData = rsp.hasOwnProperty('result') ? Utils.prettyStringify(rsp.result) : '{}';
        } catch (error) {
          console.error("getTaskInfo failed: " + error.message);
        }
      },
      async showLog() {
        if (this.path === 'log__.txt' || this.path === '') { // 尚未获取到路径
          this.log = '';
          this.path = '';
          this.$refs.logView.show();
          return;
        }

        console.log("path: " + this.path);
        let api = new Api();
        let content = await api.getFile(this.path);
        if (typeof content === 'object') {
          // 使用易读模式打印
          content = JSON.stringify(content, null, 2);
        }
        console.log("context: " + content);
        this.log = content;
        this.$refs.logView.show();
      },
      showResult() {
        // 显示实际数据
        console.log("actData: " + this.actData);
        this.log = this.actData === "{}" ? "未选择任何案例，请选择案例后再执行！" : this.actData;
        this.$refs.logView.show();
      },
      initEditor: function (editor) {
        require('brace/mode/json');
        require('brace/theme/github');

        // 禁用 ACE 编辑器的查找功能，避免控制台抛错误
        // https://stackoverflow.com/a/27797303/10366378
        editor.commands.removeCommand('find');
      },
    }
  }
</script>
<style lang="less">
  .modal-fullscreen1 .modal {
    padding: 0 !important;
  }
  .modal-fullscreen1 .modal-dialog {
    max-width: 100%;
    height: 100%;
    margin: 0;
  }
  .modal-fullscreen1 .modal-content {
    border: 0;
    border-radius: 0;
    min-height: 100%;
    height: auto;
  }

  .modal-fullscreen2 .modal {
    padding: 0 !important;
  }
  .modal-fullscreen2 .modal-dialog {
    max-width: 100%;
    height: 100%;
    margin: 0;
  }
  .modal-fullscreen2 .modal-content {
    width: calc(100% - 2rem);
    min-height: 100%;
    height: auto;
    margin: 1rem;
  }
</style>
