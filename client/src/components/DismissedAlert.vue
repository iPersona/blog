<template>
  <div>
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
  </div>
</template>
<script>
  export default {
    name: "DismissedAlert",
    components: {},
    data() {
      return {
        variant: "success",
        checkerInfo: "",
        dismissSecs: 5,
        dismissCountDown: 0,
      }
    },
    mounted() { },
    methods: {
      countDownChanged(dismissCountDown) {
        this.dismissCountDown = dismissCountDown
      },
      showError(msg, timeout) {
        if (timeout === undefined) {
          timeout = 10; // 默认超时
        }

        this.dismissCountDown = timeout
        this.checkerInfo = msg;
        this.variant = "danger";
      },
      showMsg(msg, timeout) {
        if (timeout === undefined) {
          timeout = 3;  // 默认超时
        }

        this.dismissCountDown = timeout
        this.checkerInfo = msg;
        this.variant = "success";
      },
    },
  }
</script>

