<template>
  <div>
    <b-collapse class="panel">
      <div class="panel-heading">
        <b>Personal settings</b>
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
  },
  beforeDestroy() {
  },
  methods: {
    defaultItems() {
      return [
        {
          label: 'Profile',
          to: { name: 'profile' },
          active: true,
          icon: 'id-card'
        },
        {
          label: 'Security',
          to: { name: 'security' },
          active: false,
          icon: 'lock'
        },
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