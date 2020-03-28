<template>
  <div>
    <b-table
      :data="data"
      :hoverable="true"
      :loading="isLoading"
      :mobile-cards="true"
      default-sort-direction="desc"
      default-sort="create_time"
    >
      <template slot-scope="props">
        <!-- article title -->
        <b-table-column
          field="article_id"
          label="Article"
          sortable
        >
          <router-link
            :to="toArticle(`${props.row.article_id}`, `${props.row.comment_id}`, `${props.row.id}`)"
            :class="{row: !props.row.is_read}"
            @click.native="updateNotificationNum(props.row.id)"
          >
            {{ props.row.article_title }}
          </router-link>
        </b-table-column>

        <!-- from user -->
        <b-table-column
          field="nickname"
          label="From user"
          sortable
        >
          <span :class="{row: !props.row.is_read}">
            {{ props.row.nickname }}
          </span>
        </b-table-column>

        <!-- content -->
        <b-table-column
          field="comment"
          label="Subscription"
          sortable
        >
          <span :class="{row: !props.row.is_read}">
            {{ props.row.comment }}...
          </span>
        </b-table-column>

        <!-- create time -->
        <b-table-column
          field="create_time"
          label="Date"
          sortable
        >
          <b-tag>
            <span :class="{row: !props.row.is_read}">
              {{ formatDate(props.row.create_time) }}
            </span>
          </b-tag>
        </b-table-column>
      </template>

      <template slot="empty">
        <section class="section">
          <div class="content has-text-grey has-text-centered">
            <p>
              <a @click="loadNotifications">
                <RefreshCwIcon size="2x" />
              </a>
            </p>
            <p>
              No notifications yet
              <span style="margin-left: 0.8rem;">
                Σ( ° △ °)
              </span>
            </p>
          </div>
        </section>
      </template>
    </b-table>
  </div>
</template>

<script>
import Api from '@/api'
import { RefreshCwIcon } from 'vue-feather-icons'

import { mapMutations } from 'vuex'
import { DECREASE_NOTIFICATION_NUM } from "@/store/modules/mutation-types.js"
import { USER } from '@/store/modules/module-names'

export default {
  name: 'CommentNotification',
  components: {
    RefreshCwIcon,
  },
  props: {},
  data() {
    return {
      data: [],
      isLoading: true,
    }
  },
  mounted() {
    this.loadNotifications()
  },
  methods: {
    ...mapMutations(USER, {
      decreaseNotificationNum: DECREASE_NOTIFICATION_NUM
    }),
    async loadNotifications() {
      // show loading indicator
      this.isLoading = true

      // load data from server
      let api = new Api()
      let rsp = await api.getCommentNotifications()
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`failed to get comment notifications: ${JSON.stringify(rsp.detail)}`)
        return
      }
      console.log(`comment-notifications: ${JSON.stringify(rsp.data)}`)

      // update data
      this.data = rsp.data
      // stop loading indicator
      this.isLoading = false
    },
    toArticle(articleId, commentId, ntycid) {
      return {
        name: 'article',
        params:
        {
          id: articleId
        },
        query: {
          locate: commentId,
          ntycid: ntycid,
        }
      }
    },
    updateNotificationNum(ntycid) {
      // decrease comment notification number
      let is_read = this.data.filter(t => {
        return t.id == parseInt(ntycid)
      })[0].is_read

      if (!is_read) {
        // decrease the notification number
        this.decreaseNotificationNum()
      }
    },
    formatDate(datetime) {
      let dateFormat = require('dateformat')
      let d = Date.parse(datetime)
      return dateFormat(d, "yyyy-mm-dd hh:MM:ss")
    },
  }
}
</script>
<style scoped>
.row {
  font-weight: bold;
}
</style>