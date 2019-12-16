<template>
  <article class="media">
    <figure class="media-left">
      <p class="image is-64x64">
        <Avatar />
      </p>
    </figure>
    <div class="media-content">
      <div class="bubble">
        <BField>
          <MarkdownEditor
            ref="editor"
            :min-height="100"
            placeholder="Leave a comment"
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
                Comment
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
import { EventBus, EVENT_ARTICLE_EDITOR_CLOSED, EVENT_RELOAD_COMMENTS } from '@/event-bus.js';
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
    }
  },
  data() {
    return {

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
  },
  methods: {
    listenEvent() {
      const self = this;
      EventBus.$on(EVENT_ARTICLE_EDITOR_CLOSED, async function () {
        console.log(`event-bus: ${EVENT_ARTICLE_EDITOR_CLOSED}`)
        await self.$refs.editor.setContent('')
      })
    },
    async newComment() {
      let api = new Api()
      let rsp = await api.newComment(this.articleId, this.$refs.editor.content(), this.userId)
      if (!Api.isSuccessResponse(rsp)) {
        this.$getUi().toast.fail(`failed to comment: ${rsp.detail}`)
        return
      }
      // reload comments
      EventBus.$emit(EVENT_RELOAD_COMMENTS)
      // clear old content
      this.$refs.editor.setContent('')
      // confirm to user
      this.$getUi().toast.success('You have successfully submit a comment!', true)
    }
  },
}
</script>

<style scoped src="@/assets/css/comment.css">
</style>