<template>
  <div :class="{border: true, light: isLight, dim: isLight}" />
</template>

<script>
export default {
  name: 'BlinkFocus',
  components: {},
  data() {
    return {
      isLight: false,
    }
  },
  mounted() {
    this.blink()
  },
  methods: {
    blink() {
      this.startStoppableInterval(() => {
        this.isLight = !this.isLight
      }, 500, 3)
    },
    startStoppableInterval(callback, delay, repetitions) {
      let x = 0
      let intervalID = setInterval(function () {
        callback()
        if (++x === repetitions) {
          clearInterval(intervalID)
        }
      }, delay)
    },
  },
}
</script>

<style scoped>
.border {
  transition: background 0.3s;
  border-radius: 5px;
  border-width: 0;
}

.light {
  background-color: #d5c7fc;
}

.dim {
  background-color: transparent;
}
</style>