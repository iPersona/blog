<template>
  <div>
    <b-button
      class="as-span"
      type="is-text"
      :size="size"
      :style="styleObject"
    >
      <span>{{ text }}</span>
    </b-button>
  </div>
</template>

<script>
export default {
  name: 'BadgeIcon',
  components: {},
  props: {
    size: {
      type: String,
      default: 'is-medium'
    },
    color: {
      type: String,
      default: 'black'
    },
    text: {
      type: String,
      default: ''
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
    styleObject: function () {
      let classObj = {
        'text-decoration': 'none',
        color: this.color,
        size: this.size
      }

      if (this.textSize !== '') {
        classObj['font-size'] = this.textSize
      }

      return classObj
    }
  },
  mounted() { },
  methods: {

  }
}
</script>

<style scoped>
.as-span {
  display: flex;
  align-items: center;
  padding: 5px;
  margin-left: 5px;
  /* disable pointer cursor */
  cursor: text;
}

.as-span.is-text:hover,
.as-span.is-text.is-hovered,
.as-span.is-text:focus,
.as-span.is-text.is-focused {
  /* pretend button as a span */
  background-color: transparent;
}
</style>