<template>
  <div class="container">
    <ul class="comment-list">
      <CommentEntity
        v-for="(comment, idx) in comments"
        :ref="comment.id"
        :key="comment.id"
        :comment="comment"
        :show-separator="idx !== 0"
        :location-data="getLocationData(comment.id)"
        class="comment-entity"
      />
    </ul>
    <b-pagination
      :total="totalPages"
      :current.sync="currentPage"
      :per-page="pageSize"
      order="is-centered"
      aria-next-label="Next page"
      aria-previous-label="Previous page"
      aria-page-label="Page"
      aria-current-label="Current page"
      @change="pageChanged"
    />
  </div>
</template>

<script>
import CommentEntity from './CommentEntity'
import Api from '@/api'
import Utils from '@/utils'
import { EventBus, EVENT_RELOAD_COMMENTS } from '@/event-bus.js'

import { USER } from '@/store/modules/module-names'
import { mapGetters } from 'vuex'
import { USER_ID } from '@/store/modules/store-types.js'

export default {
  name: "Comments",
  components: {
    CommentEntity,
  },
  props: {
    articleId: {
      type: String,
      default: ''
    },
    locateCommentId: {
      type: String,
      default: ''
    },
    cmtNtyId: {
      type: String,
      default: '',
    }
  },
  data() {
    return {
      comments: [],
      currentPage: 1,
      pageSize: 10,
      totalPages: 0,
      locationData: undefined,
    }
  },
  computed: {
    ...mapGetters(USER, {
      userId: USER_ID
    }),

  },
  async mounted() {
    console.log(`articleId: ${this.articleId}`)
    // load comments
    if (this.locateCommentId === '') {
      await this.loadComments(true)
    } else {
      await this.locateComment()
    }
    this.listenEventBus()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_RELOAD_COMMENTS)
  },
  methods: {
    getLocationData(commentId) {
      if (this.locationData === undefined) {
        // not locate comment
        return undefined
      }

      // get target comment id, 
      // if no `parentId` field, there is no sub comments
      let targetId = this.locationData.parentId === undefined ? this.locationData.targetId : this.locationData.parentId

      return commentId === targetId ? this.locationData : undefined
    },
    async locateComment() {
      if (Utils.isStringEmpty(this.userId)) {
        this.$getUi().toast.fail('Permission denied! Please login!')
        return
      }

      let api = new Api()
      let rsp = await api.locateComment(this.userId, this.articleId, this.locateCommentId, parseInt(this.cmtNtyId), this.pageSize)
      if (!rsp.isSuccess()) {
        console.log(`rsp-err: ${JSON.stringify(rsp)}`)
        return
      }
      console.log(`resp: ${JSON.stringify(rsp)}`)

      // update total page
      this.totalPages = rsp.data().parent.total
      this.currentPage = rsp.data().parent.page
      this.comments = rsp.data().parent.comments

      // expand sub comments
      this.locationData = {
        targetId: this.locateCommentId,
        parentId: rsp.data().child === undefined ? undefined : rsp.data().child.pid,
        child: rsp.data().child
      }
    },
    listenEventBus() {
      const self = this;
      EventBus.$on(EVENT_RELOAD_COMMENTS, async function (opt) {
        console.log(`event-bus: ${EVENT_RELOAD_COMMENTS}`)
        if (opt === undefined || opt.forceReload || self.comments.length <= 0) {
          await self.loadComments(true)
        }
      })
    },
    async loadComments(isReload) {
      console.log(`load-comments`)
      // request comments
      let api = new Api()
      let rsp = await api.getComments(this.articleId, {
        limit: this.pageSize,
        offset: (this.currentPage - 1) * this.pageSize,
      })
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`)
      if (!rsp.isSuccess()) {
        this.$getUi().toast.fail(`failed to get comments: ${rsp.errorDetail()}`)
        return
      }

      // update total page
      this.totalPages = rsp.data().total
      if (isReload) {
        // reset data
        this.comments = []
        this.currentPage = 1
      }
      // update comment data
      this.comments = rsp.data().comments
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
  margin-bottom: 0.75rem;
}
</style>