<template>
  <b-field
    grouped
    align="left"
  >
    <b-field label="Date Range">
      <b-datepicker
        v-model="dateRange"
        placeholder="Click to select..."
        icon="calendar-today"
        range
      />
    </b-field>
    <b-field label="Quick Select">
      <b-dropdown
        v-model="selectAction"
        aria-role="list"
        @change="quickSelectChanged"
      >
        <button
          slot="trigger"
          class="button is-primary"
        >
          <span>{{ selectAction }}</span>
          <b-icon icon="menu-down" />
        </button>
        <b-dropdown-item
          v-for="act in actions"
          :key="act.name"
          :value="act.name"
          aria-role="listitem"
        >
          {{ act.name }}
        </b-dropdown-item>
      </b-dropdown>
    </b-field>
  </b-field>
</template>

<script>
import { EventBus, EVENT_STATISTIC_DATE_RANGE_CHANGED } from '@/event-bus'
let moment = require("moment")

const ACTION_UNSELECT = "        "
const ACTION_LAST_30_DAYS = "Last 30 days"
const ACTION_LAST_QUARTER = "Last quarter"
const ACTION_LAST_YEAR = "Last year"

export default {
  name: 'DateRangePicker',
  components: {

  },
  props: {},
  data() {
    return {
      dateRange: [],
      selectAction: ACTION_UNSELECT,
      // actions: ["Last 30 days", "Last quarter", "Last year"],
      actions: [
        {
          name: ACTION_LAST_30_DAYS,
          from: function () {
            return moment().subtract(1, "months")
          }(),
          to: function () {
            return moment()
          }()
        },
        {
          name: ACTION_LAST_QUARTER,
          from: function () {
            return moment().subtract(3, "months")
          }(),
          to: function () {
            return moment()
          }(),
        },
        {
          name: ACTION_LAST_YEAR,
          from: function () {
            return moment().subtract(1, "years")
          }(),
          to: function () {
            return moment()
          }()
        }
      ]
    }
  },
  watch: {
    dateRange(newVal, oldVal) {
      if (newVal.length === 0) {
        // date picker value is cleared
        return
      }

      this.selectAction = ACTION_UNSELECT;  // new range selected, clear quick selection
      let from = moment(newVal[0]).format('YYYY-MM-DD')
      let to = moment(newVal[1]).format('YYYY-MM-DD')
      console.log(`from: ${from}, to: ${to}`)
      this.dateRangeChanged(from, to)
    }
  },
  mounted() {
  },
  methods: {
    getDefaultRange() {
      let act = this.actions.filter(t => t.name === ACTION_LAST_30_DAYS)[0]
      return {
        from: act.from,
        to: act.to
      }
    },
    quickSelectChanged(newVal) {
      this.selectAction = newVal
      let from = this.actions.filter(t => t.name === newVal)[0].from.format("YYYY-MM-DD")
      let to = this.actions.filter(t => t.name === newVal)[0].to.format("YYYY-MM-DD")
      this.dateRangeChanged(from, to)

      this.dateRange = [] // clear date range of date picker
    },
    dateRangeChanged(from, to) {
      EventBus.$emit(EVENT_STATISTIC_DATE_RANGE_CHANGED, {
        from,
        to
      })
    }
  },
}
</script>