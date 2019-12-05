<template>
  <div>
    <b-collapse class="panel">
      <div class="panel-heading">
        <b>Management</b>
      </div>
      <router-link
        v-for="i in items"
        :key="i.label"
        class="panel-block"
        :to="i.to"
        :active="i.active"
        @click="menuItemClicked"
      >
        <span class="panel-icon">
          <b-icon
            pack="fas"
            :icon="i.icon"
            size="is-small"
          />
        </span>
        {{ i.label }}
      </router-link>
    </b-collapse>
  </div>
</template>

<script>
import { EventBus, EVENT_STATISTIC_VIEW_MOUNTED } from '@/event-bus'

export default {
  name: "ActionMenu",
  components: {},
  data() {
    return {
      items: this.defaultItems()
    }
  },
  mounted() {
    this.listenEventBus()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_STATISTIC_VIEW_MOUNTED)
  },
  methods: {
    listenEventBus() {
      let self = this
      EventBus.$on(EVENT_STATISTIC_VIEW_MOUNTED, function () {
        self.resetItem()
      })
    },
    defaultItems() {
      return [
        {
          label: 'Dashboard',
          to: '/management/stat',
          active: true,
          icon: 'chart-bar'
        },
        {
          label: 'Articles',
          to: '/management/articles',
          active: false,
          icon: 'newspaper'
        },
        {
          label: 'Tags',
          to: '/management/tags',
          active: false,
          icon: 'tags'
        },
        {
          label: 'Settings',
          to: '/management/settings',
          active: false,
          icon: 'cog'
        }
      ]
    },
    menuItemClicked(mouseEvent) {
      this.items.forEach(item => {
        item.active = item.label.trim() === mouseEvent.currentTarget.text.trim()
      });
    },
    resetItem() {
      this.items = this.defaultItems()
    }
  }
};
</script>