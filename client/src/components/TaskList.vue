<template>
  <div>
    <dismissed-alert ref="alert" />
    <b-list-group>
      <b-list-group-item
        class="d-flex justify-content-between align-items-center"
        href="#"
        v-for="v in target"
        v-bind:key="getCaseKey(v)"
        v-bind:active="tab === getCaseKey(v)"
        @click="selectItem(v)"
      >
        <!-- 测试项名称 -->
        <div class="d-flex justify-content-between align-items-center">
          <b-badge variant="primary">
            {{v.caseSet}}
          </b-badge>
          <span class="mx-1">/</span>
          <b-badge variant="secondary">
            {{v.caseId}}
          </b-badge>
        </div>
        <!-- 状态 -->
        <b-badge :variant="getStatusBadgeVariant(v.status)">
          {{getStatusInfo(v.status)}}
        </b-badge>
        <!-- 耗时 -->
        <!-- <span v-if="v.elapse > 0">耗时：{{v.elapse}}s</span> -->
        <b-badge variant="info">{{v.elapse}}s</b-badge>
      </b-list-group-item>
    </b-list-group>
  </div>
</template>

<script>
  import Api from '@/api.js'
  import Utils from '@/utils.js'
  import DismissedAlert from './DismissedAlert.vue'

  export default {
    name: 'TaskList',
    components: {
      DismissedAlert,
    },
    props: {
      filterCondition: {}
    },
    data() {
      return {
        caseData: [],
        isRunning: false,
        taskType: "ExcAll",
        target: [],
        tab: ''
      }
    },
    mounted() {
      this.fetchData();
      this.fetchDataInBackground();
      // this.caseData = this.getDemoData();
      // 数据更新后，触发数据变更事件
      this.updateData();
    },
    watch: {
      filterCondition: function (newVal, oldVal) {
        this.filterData();
        if (this.target.length === 0) {
          this.$refs.alert.showError('未查询到匹配的用例！', 3);
        }
      }
    },
    methods: {
      fetchDataInBackground() {
        var thiz = this;
        setInterval(async function () {
          await thiz.fetchData();
        }, 10000);  // 默认10秒刷新一次数据
      },
      async fetchData() {
        try {
          let api = new Api();
          let rsp = await api.getTask();
          this.caseData = rsp.cases;
          this.isRunning = rsp.isRunning;
          this.taskType = rsp.taskType;
          console.log("rsp-task: " + JSON.stringify(rsp));

          // 数据更新后，触发数据变更事件
          this.updateData();
        } catch (error) {
          console.error("getCaseDescription failed: " + error.message);
        }
      },
      // getDemoData() {
      //   return [
      //     { caseId: "case-001", caseSet: "PBOC测试案例", elapse: 0, status: "Success" },
      //     { caseId: "case-002", caseSet: "PBOC测试案例", elapse: 0, status: "Failed" },
      //     { caseId: "case-003", caseSet: "PBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-004", caseSet: "PBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-005", caseSet: "PBOC测试案例", elapse: 0, status: "Running" },
      //     { caseId: "case-001", caseSet: "QPBOC测试案例", elapse: 0, status: "Success" },
      //     { caseId: "case-002", caseSet: "QPBOC测试案例", elapse: 0, status: "Failed" },
      //     { caseId: "case-003", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-004", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-005", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-006", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-007", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-008", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-009", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-010", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-011", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-012", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-013", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-014", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-015", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-016", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-017", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-018", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-019", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-020", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-021", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-022", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-023", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-024", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-025", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-026", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-027", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-028", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-029", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-030", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-031", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-032", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-033", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" },
      //     { caseId: "case-034", caseSet: "QPBOC测试案例", elapse: 0, status: "NotRun" }
      //   ]
      // },
      getCaseKey(caseObj) {
        return 'cid-' + Utils.getObjectHash(caseObj);
      },
      getStatusBadgeVariant(status) {
        let vMap = {
          Success: 'success',
          Failed: 'danger',
          NotRun: this.isRunning ? 'warning' : 'light',
          Running: 'primary',
        }
        let v = vMap[status];
        return v;
      },
      getStatusInfo(status) {
        let vMap = {
          Success: '成功',
          Failed: '失败',
          NotRun: this.isRunning ? '排队中' : '未执行',
          Running: '正在运行',
        }
        return vMap[status];
      },
      selectItem(v) {
        this.tab = this.getCaseKey(v)
        let args = {
          caseSet: v.caseSet,
          caseId: v.caseId,
        }
        this.$emit('selectedItemChanged', args);
      },
      updateData() {
        this.filterData();  // 过滤用来显示的数据
        let args = {
          cases: this.caseData,
          isRunning: this.isRunning,
          taskType: this.taskType,
        }
        this.$emit('dataChanged', args);
      },
      filterData() {
        let target = this.caseData;

        if (this.filterCondition.hasOwnProperty('caseSet')) {
          target = target.filter(item => {
            return item.caseSet === this.filterCondition.caseSet;
          })
        }

        if (this.filterCondition.hasOwnProperty('caseId')) {
          let caseIds = Utils.getCaseIdFromQueryString(this.filterCondition['caseId']);
          console.log("caseIds: " + caseIds);
          target = target.filter(item => {
            // return item.caseId === this.filterCondition.caseId;
            return caseIds.includes(item.caseId.trim());
          })
        }

        if (this.filterCondition.hasOwnProperty('status')) {
          if (this.filterCondition.status !== "All") { // 切换回全状态案例显示时，不能过滤 All 这个特殊值
            target = target.filter(item => {
              return item.status === this.filterCondition.status;
            })
          }
        }

        this.target = target; // 最后赋值，防止多次更新变量造成的行能开销
      }
    }
  }
</script>

