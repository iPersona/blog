<template>
  <div>
    <b-collapse class="panel">
      <div class="panel-heading">
        <b>Notifications</b>
      </div>
      <router-link
        v-for="(item, idx) in items"
        :key="item.label"
        class="panel-block"
        :to="item.to"
        :active="status[idx]"
        @click="menuItemClicked"
      >
        <template>
          <!-- eslint-disable vue/no-v-html -->
          <span
            class="panel-icon menu_icon"
            v-html="toSvg(item.icon)"
          />
          <!-- </span> -->
          <span class="menu_text">
            {{ item.label }}
          </span>
        </template>
      </router-link>
    </b-collapse>
  </div>
</template>

<script>
import { EventBus, EVENT_RESET_ACTION_MENU_STATUS } from '@/event-bus'

export default {
  name: "ActionMenu",
  components: {},
  props: {
    items: {
      type: Array,
      default: () => [],
    }
  },
  data() {
    return {
      status: []
    }
  },
  watch: {
    items: {
      immediate: true,
      handler(newVal, oldVal) {
        // Reset status when items prop changed
        this.status = Array(newVal.length).fill(false)
      }
    }
  },
  mounted() {
    this.listenEventBus()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_RESET_ACTION_MENU_STATUS)
  },
  methods: {
    listenEventBus() {
      let self = this
      EventBus.$on(EVENT_RESET_ACTION_MENU_STATUS, function () {
        // Reset status when received reset event
        self.status = Array(self.items.length).fill(false)
      })
    },
    menuItemClicked(mouseEvent) {
      this.items.forEach((item, idx) => {
        this.status[idx] = item.label.trim() === mouseEvent.currentTarget.text.trim()
      });
    },
    toSvg(name) {
      let feather = require('feather-icons')
      return feather.icons[name].toSvg()
    }
  }
};
</script>

<style scoped>
.menu_icon {
  display: inline-block;
  vertical-align: middle;
  height: 1.7em;
}

.menu_text {
  margin-left: 0.5rem;
}
</style>