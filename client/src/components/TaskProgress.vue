<template>
  <div>
    <br />
    <h5 class="text-left">执行进度</h5>
    <b-progress
      :value="max-pending-running"
      :max="max"
      show-progress
      animated
    ></b-progress>
    <br>
    <h5 class="text-left">用例状态</h5>
    <b-progress
      class="mt-1"
      :max="max"
      show-value
    >
      <!-- 成功用例 -->
      <b-progress-bar
        id="pbSuccess"
        :value="success"
        variant="success"
      ></b-progress-bar>

      <!-- 失败用例 -->
      <b-progress-bar
        id="pbFailed"
        :value="failed"
        variant="danger"
      ></b-progress-bar>

      <!-- 排队用例 -->
      <b-progress-bar
        id="pbPending"
        :value="pending"
        variant="warning"
      ></b-progress-bar>

      <!-- 执行中的用例 -->
      <b-progress-bar
        id="pbRunning"
        :value="running"
        variant="primary"
        v-show="running > 0"
      ></b-progress-bar>

    </b-progress>

    <!-- 提示 -->
    <b-tooltip
      target="pbSuccess"
      placement="bottom"
    >成功用例数：<strong>{{success}}</strong></b-tooltip>
    <b-tooltip
      target="pbFailed"
      placement="bottom"
    >失败用例数：<strong>{{failed}}</strong></b-tooltip>
    <b-tooltip
      target="pbPending"
      placement="bottom"
    >尚未执行用例数：<strong>{{pending}}</strong></b-tooltip>
    <b-tooltip
      target="pbRunning"
      placement="bottom"
    >执行中用例数：<strong>{{running}}</strong></b-tooltip>
    <div class="d-flex">总用例数：{{ max}}</div>

  </div>
</template>
<script>
  export default {
    name: "TaskProgress",
    components: {
    },
    data() {
      return {
        max: 0,
        success: 0,
        failed: 0,
        pending: 0,
        running: 0,
        isRunning: false,
        startBtnText: '开始',
        startBtnVariant: 'success',
        showMsg: false,
      };
    },
    watch: {
      isRunning: async function (newVal, oldVal) {
        this.startBtnText = newVal === true ? "停止" : "开始"
        this.startBtnVariant = newVal === true ? "danger" : "success"
      }
    },
    methods: {
      updateProgress(args) {
        this.max = args.max;
        this.success = args.success;
        this.failed = args.failed;
        this.pending = args.pending;
        this.isRunning = args.isRunning;
        this.running = this.isRunning === true ? 1 : 0;

        if (this.showMsg) {
          this.$refs.alert.showMsg(this.isRunning === true ? "开始运行用例" : "用例执行已停止");
          this.showMsg = false;
        }
      }
    }
  }
</script>
<style scoped>
</style>


