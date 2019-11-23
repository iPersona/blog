<template>
  <div>
    <BMenu label="Menu">
      <BMenuList>
        <BMenuItem
          v-for="i in items"
          :key="i.label"
          :label="i.label"
          tag="router-link"
          :to="i.to"
          icon-pack="fas"
          :icon="i.icon"
          :active="i.active"
          @click="menuItemClicked"
        />
      </BMenuList>
    </BMenu>
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