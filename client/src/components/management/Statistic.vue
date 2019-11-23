<template>
  <div>
    <section>
      <date-range-picker ref="dateRangePicker" />
    </section>
    <section>
      <vue-apex-charts
        type="area"
        :options="chartOptions"
        :series="series"
      />
    </section>
  </div>
</template>

<script>
import Api from "@/api"
import VueApexCharts from "vue-apexcharts"
import DateRangePicker from "./DateRangePicker"
import { EventBus, EVENT_STATISTIC_DATE_RANGE_CHANGED, EVENT_STATISTIC_VIEW_MOUNTED } from '@/event-bus'

export default {
  name: "Statistic",
  components: {
    VueApexCharts,
    DateRangePicker,
  },
  data() {
    return {
      chartOptions: {
        dataLabels: {
          enabled: false
        },
        stroke: {
          curve: "smooth"
        },
        tooltip: {
          fixed: {
            enabled: false,
            position: "topRight"
          }
        },
        // xaxis: {
        //   type: "datetime"
        // }
      },
      series: []
    }
  },
  async mounted() {
    this.emitMountedEvent()
    this.listenEventBus()
    await this.loadDefaultRange()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_STATISTIC_DATE_RANGE_CHANGED)
  },
  methods: {
    emitMountedEvent() {
      EventBus.$emit(EVENT_STATISTIC_VIEW_MOUNTED)
    },
    async loadDefaultRange() {
      let range = this.$refs.dateRangePicker.getDefaultRange()
      await this.loadDailyVisits(range.from, range.to)
    },
    listenEventBus() {
      const self = this
      EventBus.$on(EVENT_STATISTIC_DATE_RANGE_CHANGED, async function (val) {
        await self.loadDailyVisits(val.from, val.to)
      })
    },
    async loadDailyVisits(from, to) {
      let api = new Api()
      let rsp = await api.getDailyPeriod(from, to)
      if (!Api.isSuccessResponse(rsp)) {
        this.$getLog().debug(`getDailyPeriod failed: ${rsp.detail}`)
      }
      this.$getLog().debug(`getDailyPeriod success: ${JSON.stringify(rsp.data)}`)

      let data = this.splitData(rsp.data)
      this.series = [
        {
          name: "visit times",
          data: data
        }
      ]
    },
    splitData(data) {
      let moment = require("moment")
      return data.map(i => {
        i.today = moment(i.today).format("MM/DD/YYYY")
        return {
          x: i.today,
          y: i.visit_num
        }
      })
    }
  }
}
</script>
