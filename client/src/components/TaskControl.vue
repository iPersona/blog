<template>
  <div>
    <div class="d-flex justify-content-between">
      <!-- 数据库初始化按钮 -->
      <b-button
        variant="primary"
        @click="reloadDb"
      >初始化用例数据库</b-button>

      <!-- 执行部分案例 控制按钮 -->
      <b-button-group>
        <b-btn
          :variant="startBtnVariant"
          @click="startTest"
        >{{startBtnText}}</b-btn>
        <b-button
          :variant="startCurrentBtnVariant"
          @click="startSelect"
        >{{startCurrentBtnText}}</b-button>
        <b-button
          :variant="startFilterBtnVariant"
          @click="startFilter"
        >{{startFilterBtnText}}</b-button>
      </b-button-group>

      <!-- 功能按钮 -->
      <b-btn
        variant="primary"
        @click="exportData"
      >导出测试数据</b-btn>
    </div>
    <dismissed-alert ref="alert" />
  </div>
</template>
<script>
  import Api from '@/api.js'
  import DismissedAlert from './DismissedAlert.vue'
  import FileSaver from 'file-saver'

  export default {
    name: "Task",
    components: {
      DismissedAlert,
    },
    data() {
      return {
        cases: [],
        selectedCaseSet: '',
        selectedCaseId: '',
        filterCases: [],
        isRunning: false,
        taskType: 'ExcAll',
        startBtnText: '【全部执行】',
        startBtnVariant: 'success',
        startCurrentBtnText: '【执行当前案例】',
        startCurrentBtnVariant: 'success',
        startFilterBtnText: '【执行筛选案例】',
        startFilterBtnVariant: 'success',
        showMsg: false,
      };
    },
    watch: {
      isRunning: async function (newVal, oldVal) {
        console.log("taskType: " + this.taskType);
        if (this.taskType === 'ExcAll') {
          this.startBtnText = newVal === true ? "【停止】" : "【全部执行】"
          this.startBtnVariant = newVal === true ? "danger" : "success"
        }

        if (this.taskType === 'ExcCurrent') {
          this.startCurrentBtnText = newVal === true ? "【停止】" : "【执行当前案例】"
          this.startCurrentBtnVariant = newVal === true ? "danger" : "success"
        }

        if (this.taskType === 'ExcFilter') {
          this.startFilterBtnText = newVal === true ? "【停止】" : "【执行筛选案例】"
          this.startFilterBtnVariant = newVal === true ? "danger" : "success"
        }
      }
    },
    methods: {
      updateStatus(args) {
        console.log("updateStatus: args: " + JSON.stringify(args));
        if (args.cases !== undefined) {
          this.cases = args.cases;
        }
        if (args.isRunning !== undefined) {
          this.isRunning = args.isRunning;
        }
        if (args.taskType !== undefined) {
          this.taskType = args.taskType;
        }

        if (this.showMsg) {
          this.$refs.alert.showMsg(this.isRunning === true ? "开始运行用例" : "用例执行已停止");
          this.showMsg = false;
        }
      },
      updateCurrentCase(args) {
        this.selectedCaseSet = args.caseSet;
        this.selectedCaseId = args.caseId;
      },
      updateFilterCases(cases) {
        this.filterCases = cases;
      },
      async startTest() {
        this.showMsg = true;

        let api = new Api();
        let isStart = !this.isRunning;
        await api.controlTask(isStart, 'ExcAll');
      },
      async startSelect() {
        console.log("startSelect");
        // 执行当前选中案例
        if (this.selectedCaseSet === '' || this.selectedCaseId === '') {
          this.$refs.alert.showMsg("未选择任何案例，请从列表中选择案例后再执行！");
          return;
        }

        let api = new Api();
        let isStart = !this.isRunning;
        let args = [{
          caseSet: this.selectedCaseSet,
          cases: [this.selectedCaseId],
        }];
        await api.controlTask(isStart, 'ExcCurrent', args);
      },
      async startFilter() {
        console.log("startFilter");
        // 执行列表过滤出的案例
        if (this.filterCases === undefined || this.filterCases.length === 0) {
          this.$refs.alert.showMsg("未筛选案例，请筛选案例后再执行！");
          return;
        }

        let api = new Api();
        let isStart = !this.isRunning;
        await api.controlTask(isStart, 'ExcFilter', this.filterCases);
      },
      async exportData() {
        let api = new Api();
        let rsp = await api.getResults();

        console.log("exportdata-rsp: " + JSON.stringify(rsp));
        var blob = new Blob([JSON.stringify(rsp)], { type: "text/plain;charset=utf-8" });
        FileSaver.saveAs(blob, 'data.json');
      },
      async reloadDb() {
        // 首次获取数据时，可能会发生没有任何数据的情况，
        // 这是由于服务端还没有向数据库载入用例数据的原因
        console.log("reload cases...");
        this.$refs.alert.showMsg("初始化用例数据库，请稍后！");
        let api = new Api();
        await api.reloadCases();
        this.$refs.alert.showMsg("例数据库初始化完成");
      }
    }
  }
</script>
<style scoped>
</style>


