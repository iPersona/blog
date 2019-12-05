<template>
  <div class="container">
    <ul class="comment-list">
      <CommentEntity
        v-for="comment in comments"
        :key="comment.id"
        :comment="comment"
        class="comment-entity"
      />
    </ul>
    <InfiniteLoading
      ref="infiniteLoading"
      spinner="waveDots"
      @infinite="infiniteHandler"
    >
      <div slot="no-more" />
      <div slot="no-results">
        No comments
      </div>
    </InfiniteLoading>
  </div>
</template>

<script>
// TODO: 评论树制作：https://bulma.io/documentation/layout/media-object/
import InfiniteLoading from 'vue-infinite-loading'
import CommentEntity from './CommentEntity'
import Api from '@/api'
import { EventBus, EVENT_RELOAD_COMMENTS } from '@/event-bus.js'

export default {
  name: "Comments",
  components: {
    InfiniteLoading,
    CommentEntity,
  },
  props: {
    articleId: {
      type: String,
      default: ''
    }
  },
  data() {
    return {
      comments: [],
      currentPage: 1,
      pageSize: 10,
    }
  },
  mounted() {
    console.log(`articleId: ${this.articleId}`)
    this.listenEventBus()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_RELOAD_COMMENTS)
  },
  methods: {
    listenEventBus() {
      const self = this;
      EventBus.$on(EVENT_RELOAD_COMMENTS, async function () {
        console.log(`event-bus: ${EVENT_RELOAD_COMMENTS}`)
        await self.loadComments()
        // self.$refs.infiniteLoading.stateChanger.reset()
      })
    },
    async loadComments($state) {
      console.log(`load-comments: ${$state}`)
      let api = new Api()
      let currentPage = $state === undefined ? 1 : this.currentPage
      let rsp = await api.getComments(this.articleId, this.pageSize, (currentPage - 1) * this.pageSize)
      this.$getLog().debug(`rsp: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`get comments failed: ${rsp.detail}`)
        return
      }

      if ($state === undefined) {
        this.comments = []
        this.currentPage = currentPage;
      }

      if (rsp.data.length > 0) {
        this.currentPage += 1;
        this.comments.push(...rsp.data)

        this.$getLog().debug(`state-loaded`)
        if ($state === undefined) {
          return
        }
        $state.loaded()
      } else {
        this.$getLog().debug(`state-complete`)
        if ($state === undefined) {
          return
        }
        $state.complete()
      }
    },
    async infiniteHandler($state) {
      this.loadComments($state)
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
  margin-top: 30px;
}
</style>