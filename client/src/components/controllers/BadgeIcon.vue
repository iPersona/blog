<template>
  <div
    v-if="number > 0"
    class="badge"
    :style="badgeSize"
    :data-badge="number"
    @click="clickEvent"
  >
    <!-- eslint-disable vue/no-v-html -->
    <span
      class="menu_icon"
      @click="clickEvent"
      v-html="toSvg(icon)"
    />
  </div>
  <div
    v-else
    :style="badgeSize"
  >
    <!-- eslint-disable vue/no-v-html -->
    <span
      class="menu_icon"
      @click="clickEvent"
      v-html="toSvg(icon)"
    />
  </div>
</template>

<script>
export default {
  name: 'BadgeIcon',
  components: {},
  props: {
    number: {
      type: Number,
      default: 0
    },
    icon: {
      type: String,
      default: ''
    },
    size: {
      type: String,
      default: '24',
    },
    strokeWidth: {
      type: String,
      default: 'default',
    },
    strokeLinecap: {
      type: String,
      default: 'round'
    },
    strokeLinejoin: {
      type: String,
      default: 'round'
    },
    color: {
      type: String,
      default: 'black'
    },
  },
  data() {
    return {

    }
  },
  computed: {
    // adjust outer badge div height to svg size to center the icon
    badgeSize: function () {
      let classObj = {}
      classObj['height'] = this.toCustomSize(this.size).height

      return classObj
    }
  },
  mounted() { },
  methods: {
    toSvg(name) {
      let feather = require('feather-icons')
      let customSize = this.toCustomSize(this.size)
      return feather.icons[name].toSvg({
        width: customSize.width,
        height: customSize.height,
        'stroke-width': this.toCustomStrokeWidth(this.strokeWidth),
        'stroke-linecap': this.strokeLinecap,
        'stroke-linejoin': this.strokeLinejoin,
        color: this.color
      })
    },
    toCustomSize(size) {
      let fixedSize = size.slice(-1) === 'x'
        ? size.slice(0, size.length - 1) + 'em'
        : parseInt(size) + 'px';
      return {
        width: fixedSize,
        height: fixedSize,
      }
    },
    toCustomStrokeWidth(width) {
      let val = 2 // default value 2
      if (width.toLowerCase() === 'thin') {
        val = 1
      } else if (width.toLowerCase() === 'bold') {
        val = 3
      } else if (width.toLowerCase() === 'x-bold') {
        val = 4
      } else if (width.toLowerCase() === 'xx-bold') {
        val = 5
      } else if (width.toLowerCase() === 'xxx-bold') {
        val = 6
      }

      return val
    },
    clickEvent() {
      this.$emit('clickEvent')
    }
  }
}
</script>

<style scoped>
.badge {
  display: inline-block;
  position: relative;
}

.badge[data-badge]:after {
  content: attr(data-badge);
  position: absolute;
  top: -0.1rem;
  right: -0.1rem;
  font-size: 0.7em;
  background: #ff3860;
  color: white;
  width: 15px;
  height: 15px;
  text-align: center;
  line-height: 15px;
  border-radius: 50%;
  box-shadow: 0 0 1px #333;
}
</style>