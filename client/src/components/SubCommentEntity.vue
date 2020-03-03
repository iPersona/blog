<template>
  <li ref="comment">
    <hr class="comment-spliter">
    <article
      :class="{media: true, blink: true, overlay: showOverlay}"
      style="border-top: 0px;"
    >
      <figure
        class="media-left"
        style="margin-right: 0.1rem"
      >
        <p class="image is-32x32">
          <Avatar />
        </p>
      </figure>
      <div class="media-content">
        <div>
          <BField style="margin-bottom: 0rem;">
            <!-- header -->
            <div class="comment-header">
              <!-- left header -->
              <div
                align="left"
                class="comment-header-left"
              >
                <!-- nick name -->
                <a class="is-primary comment-nickname">{{ comment.from_nickname }}</a>
              </div>
            </div>
          </BField>
          <BField style="margin-bottom: 0rem;">
            <div
              align="left"
              class="comment-content"
            >
              <MarkdownPreview :content="comment.comment" />
            </div>
          </BField>

          <!-- footer -->
          <!-- make the vertical space smaller -->
          <BField style="margin-bottom: 0px;">
            <div class="comment-status">
              <div class="comment-status-left">
                <span class="comment-info">{{ createTime }}</span>
              </div>
              <div
                v-if="isLogin"
                class="comment-status-middle"
              >
                <b-button
                  slot="trigger"
                  type="is-text"
                  size="is-small"
                  icon-pack="mdi"
                  icon-left="reply"
                  style="text-decoration: none; color: gray;"
                  aria-controls="replyComment"
                  @click="quoteReply"
                >
                  Reply
                </b-button>
              </div>
            </div>
          </BField>
          <!-- reply component -->
          <b-collapse
            :open="isReply"
            position="is-bottom"
            aria-id="replyComment"
          >
            <div
              :is="replyComponent"
              v-if="isLogin"
              v-bind="{articleId: comment.article_id, commentId: comment.id}"
              class="reply-comment"
            />
          </b-collapse>
        </div>
      </div>
    </article>
  </li>
</template>

<script>
import MarkdownPreview from './MarkdownPreview'
import NewComment from './NewComment'
import Empty from './Empty'
import Avatar from "./Avatar"
import DatetimeUtil from "./utils/datetime"
import { EventBus, EVENT_SET_COMMENT_EDITOR_CONTENT, EVENT_SCROLL_TO_COMMENT_EDITOR, EVENT_CLOSE_COMMENT_REPLY_VIEW, EVENT_CLOSE_SUB_COMMENT_REPLY_VIEW } from '@/event-bus.js'
import Url from './utils/url'
import { mapGetters } from "vuex";
import { IS_LOGIN, USER_ID } from "@/store/modules/store-types.js";
import { USER } from "@/store/modules/module-names";
import Utils from '@/utils'
import VueScrollTo from 'vue-scrollto'

export default {
  name: "SubCommentEntity",
  components: {
    MarkdownPreview,
    Avatar,
    NewComment,
    Empty,
  },
  props: {
    comment: {
      type: Object,
      default: () => { return {} }
    },
    focus: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      replyComponent: 'Empty',
      isReply: false,
      showOverlay: false,
    }
  },
  computed: {
    createTime: function () {
      return DatetimeUtil.toTimeAgo(this.comment.create_time)
    },
    ...mapGetters(USER, {
      isLogin: IS_LOGIN,
      userId: USER_ID,
    })
  },
  mounted() {
    this.listenEvents()
    if (this.focus) {
      this.locateComment()
    }
  },
  beforeDestroy() {
    EventBus.$off(EVENT_CLOSE_SUB_COMMENT_REPLY_VIEW)
  },
  methods: {
    locateComment() {
      // scroll to comment
      // the target element need some time to be mounted
      setTimeout(() => {
        // blink
        Utils.blink(() => {
          this.showOverlay = !this.showOverlay
        }, 800, 3)

        // scroll to target comment
        VueScrollTo.scrollTo(this.$refs.comment, 500)
      }, 2000)
    },
    listenEvents() {
      const self = this;
      EventBus.$on(EVENT_CLOSE_SUB_COMMENT_REPLY_VIEW, function (commentId) {
        console.log(`event-bus: ${EVENT_CLOSE_SUB_COMMENT_REPLY_VIEW}`)
        // Close comment reply window after finish commenting
        if (commentId === self.comment.id) {
          self.toggleReplyCommentView()
        }
      })
    },
    quoteReply() {
      this.toggleReplyCommentView()
    },
    toggleReplyCommentView() {
      this.isReply = !this.isReply;
      this.replyComponent = this.isReply ? 'NewComment' : 'Empty'
    },
  },
}
</script>

<style scoped>
.blink {
  transition: background 0.3s;
}

.overlay {
  background-color: #d5c7fc;
  opacity: 0.8;
  border-radius: 0.3rem;
}

.comment-spliter {
  height: 1px;
  width: 80%;
  margin: 0 0;
}

.comment-header {
  display: flex;
  padding-left: 5px;
  padding-bottom: 0%;
}

.comment-header-left {
  align-items: center;
  float: left;
}

.comment-header-right {
  float: right;
}

.sub-comments {
  display: flex;
  padding-left: 5px;
  padding-bottom: 0%;
}

.sub-comment-left {
  align-items: center;
  float: left;
}

.comment-status {
  display: flex;
  padding-left: 0rem;
  padding-bottom: 0rem;
}

.comment-status-left {
  align-items: center;
  float: left;
}

.comment-status-middle {
  align-items: center;
  float: left;
  margin-left: 20px;
}

.comment-status-right {
  float: right;
}

.comment-info {
  display: flex;
  align-items: center;
  color: gray;
  font-size: small;
  padding: 5px;
  margin-left: 5px;
}

.sub-comment-link {
  display: flex;
  align-items: center;
  font-size: small;
  padding: 5px;
  margin-left: 5px;
}

.comment-nickname {
  color: #7957d5;
  font-size: small;
  padding: 5px;
  font-weight: bold;
}

.reply-comment {
  margin-top: 0.75rem;
  margin-bottom: 0.75rem;
}

.comment-content {
  padding-left: 0.05rem;
}

/* this editor style is use to recover the p element style impacted by Buefy */
.comment-content >>> p {
  margin: 5px;
}
</style>

<style scoped src="@/assets/css/comment_item.css">
</style>