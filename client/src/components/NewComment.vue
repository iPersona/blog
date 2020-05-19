<template>
  <article class="media">
    <figure class="media-left">
      <p class="image is-64x64">
        <Avatar />
      </p>
    </figure>
    <div class="media-content">
      <div>
        <BField>
          <MarkdownEditor
            ref="editor"
            :min-height="100"
            :placeholder="placeholder"
            toolbar-style="Comment"
          />
        </BField>
        <nav class="level">
          <div class="level-left" />
          <div class="level-right">
            <div class="level-item">
              <BButton
                class="is-primary"
                @click="newComment"
              >
                {{ buttonText }}
              </BButton>
            </div>
          </div>
        </nav>
      </div>
    </div>
  </article>
</template>

<script>
import { mapGetters } from 'vuex'
import MarkdownEditor from './MarkdownEditor'
import Avatar from "./Avatar"
import { EventBus, EVENT_ARTICLE_EDITOR_CLOSED, EVENT_RELOAD_COMMENTS, EVENT_SET_COMMENT_EDITOR_CONTENT, EVENT_CLOSE_COMMENT_REPLY_VIEW, EVENT_CLOSE_SUB_COMMENT_REPLY_VIEW } from '@/event-bus.js';
import Api from '@/api.js'
import { USER_ID } from '@/store/modules/store-types.js'
import { USER } from '@/store/modules/module-names'

export default {
  name: 'NewComment',
  components: {
    MarkdownEditor,
    Avatar,
  },
  props: {
    articleId: {
      type: String,
      default: ''
    },
    nickName: {
      type: String,
      default: ''
    },
    commentId: {
      type: String,
      default: ''
    }
  },
  data() {
    return {
      buttonText: this.commentId === '' ? 'Comment' : 'Reply',
      placeholder: this.commentId === '' ? 'Leave a comment' : 'Reply this comment'
    }
  },
  computed: {
    ...mapGetters(USER, {
      userId: USER_ID
    }),
  },
  mounted() {
    console.log('NewComment mounted')
    this.listenEvent()
  },
  beforeDestroy() {
    EventBus.$off(EVENT_ARTICLE_EDITOR_CLOSED)
    EventBus.$off(EVENT_SET_COMMENT_EDITOR_CONTENT)
  },
  methods: {
    listenEvent() {
      const self = this;
      EventBus.$on(EVENT_ARTICLE_EDITOR_CLOSED, async function () {
        console.log(`event-bus: ${EVENT_ARTICLE_EDITOR_CLOSED}`)
        await self.$refs.editor.setContent('')
      })

      EventBus.$on(EVENT_SET_COMMENT_EDITOR_CONTENT, async function (content) {
        console.log(`event-bus: ${EVENT_SET_COMMENT_EDITOR_CONTENT}`)
        await self.$refs.editor.setContent(content)
      })
    },
    async newComment() {
      console.log(`comment-content: ${this.$refs.editor.content()}`)
      if (this.$refs.editor.content() === '') {
        this.$getUi().toast.warning(`Comment can not be empty!`)
        return
      }

      let api = new Api()
      let rsp = await api.newComment(this.articleId, this.$refs.editor.content(), this.userId)
      if (!rsp.isSuccess()) {
        this.$getUi().toast.fail(`failed to comment: ${rsp.errorDetail()}`)
        return
      }
      // reload comments
      EventBus.$emit(EVENT_RELOAD_COMMENTS, { forceReload: true })
      // clear old content
      this.$refs.editor.setContent('')
      // confirm to user
      this.$getUi().toast.success('You have successfully submit a comment!', true)
      // close reply window, NOT new comment component!
      if (this.commentId !== '') {
        EventBus.$emit(EVENT_CLOSE_COMMENT_REPLY_VIEW, this.commentId)
        EventBus.$emit(EVENT_CLOSE_SUB_COMMENT_REPLY_VIEW, this.commentId)
      }
    }
  },
}
</script>

<style scoped src="@/assets/css/comment.css">
</style>