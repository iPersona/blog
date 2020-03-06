<template>
  <div>
    <!-- eslint-disable vue/no-v-html -->
    <span
      class="menu_icon"
      v-html="toSvg(icon)"
    />
    <!-- </span> -->
    <span :class="menuText">
      {{ text }}
    </span>
  </div>
</template>

<script>
export default {
  name: 'IconText',
  components: {},
  props: {
    icon: {
      type: String,
      default: ''
    },
    text: {
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
    textColor: {
      type: String,
      default: 'black'
    },
    textSize: {
      type: String,
      default: ''
    }
  },
  data() {
    return {

    }
  },
  computed: {
    menuText: function () {
      let classObj = {}
      classObj['color'] = this.textColor
      if (this.textSize !== '') {
        classObj['font-size'] = this.textSize
      }
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
  }
}
</script>

<style scoped>
.menu_icon {
  display: inline-block;
  vertical-align: middle;
}
</style>