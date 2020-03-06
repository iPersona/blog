<template>
  <div class="container">
    <ul class="comment-list">
      <SubCommentEntity
        v-for="comment in comments"
        :ref="`cmt-${comment.id}`"
        :key="comment.id"
        :comment="comment"
        :focus="focus(comment.id)"
      />
    </ul>
    <b-pagination
      size="is-small"
      :total="totalPages"
      :current.sync="currentPage"
      :per-page="pageSize"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
      @change="pageChanged"
    />
  </div>
</template>

<script>
import SubCommentEntity from './SubCommentEntity'
import Api from '@/api'
import Utils from '@/utils'
import { EventBus, EVENT_RELOAD_SUB_COMMENTS } from '@/event-bus.js'

export default {
  name: "SubComments",
  components: {
    SubCommentEntity,
  },
  props: {
    articleId: {
      type: String,
      default: ''
    },
    parentCommentId: {
      type: String,
      default: ''
    },
    locationData: {
      type: Object,
      default: () => { return {} }
    }
  },
  data() {
    return {
      comments: [],
      currentPage: 1,
      pageSize: 10,
      totalPages: 0,
    }
  },
  mounted() {
    this.listenEventBus()
    if (Utils.isObjEmpty(this.locationData)) {
      // load comments
      this.loadComments(true)
    } else {
      // locate comment
      this.locateComment()
    }
  },
  beforeDestroy() {
    EventBus.$off(EVENT_RELOAD_SUB_COMMENTS)
  },
  methods: {
    listenEventBus() {
      const self = this;
      EventBus.$on(EVENT_RELOAD_SUB_COMMENTS, async function (opt) {
        console.log(`event-bus: ${EVENT_RELOAD_SUB_COMMENTS}`)
        if (opt === undefined || opt.forceReload || self.comments.length <= 0) {
          await self.loadComments(true)
        }
      })
    },
    commentRef(commentId) {
      return `cmt-${commentId}`
    },
    focus(commentId) {
      return this.locationData.child !== undefined && this.locationData.targetId === commentId
    },
    locateComment() {
      this.totalPages = this.locationData.child.total
      this.currentPage = this.locationData.child.page
      this.comments = this.locationData.child.comments
    },
    async loadComments(isReload) {
      // request comments
      let api = new Api()
      let args = {
        limit: this.pageSize,
        offset: (this.currentPage - 1) * this.pageSize,
        parent_comment: this.parentCommentId,
      }
      let rsp = await api.getComments(this.articleId, args)
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`get comments failed: ${rsp.detail}`)
        return
      }

      // update total page
      this.totalPages = rsp.data.total
      if (isReload) {
        // reset data
        this.comments = []
        this.currentPage = 1
      }
      // update comment data
      this.comments = rsp.data.comments
    },
    pageChanged(currentPage) {
      // update current page
      this.currentPage = currentPage
      this.loadComments(false)
    }
  },
}
</script>

<style scoped>
.comment-list {
  padding: 1em 0;
  margin-bottom: 15px;
}

.comment-entity {
  margin-bottom: 30px;
}
</style>