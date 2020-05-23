<template>
  <li ref="comment">
    <hr
      v-if="showSeparator"
      class="separator"
    >
    <div class="container comment-area">
      <article :class="{overlay: showOverlay, blink: true, media: true}">
        <figure class="media-left">
          <p class="image is-64x64">
            <Avatar />
          </p>
        </figure>
        <div class="media-content">
          <div>
            <BField style="margin-bottom: 0;">
              <!-- header -->
              <div class="comment-header">
                <!-- left header -->
                <div
                  align="left"
                  class="comment-header-left"
                >
                  <!-- nick name -->
                  <b-dropdown aria-role="list">
                    <b-button
                      slot="trigger"
                      type="is-text"
                      size="is-small"
                      style="text-decoration: none; color: gray; padding-left: 5px"
                    >
                      <span class="comment-nickname">{{ comment.nickname }}</span>
                    </b-button>
                    <b-dropdown-item
                      aria-role="listitem"
                      @click="copyUserInfo"
                    >
                      <IconText
                        icon="copy"
                        text="Copy user info"
                        size="1x"
                        stroke-width="bold"
                      />
                    </b-dropdown-item>
                  </b-dropdown>
                </div>
              </div>
            </BField>
            <BField style="margin-bottom: 0;">
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
                  <ButtonSpan
                    size="is-small"
                    color="gray"
                    :text="createTime"
                  />
                </div>
                <div
                  v-if="isLogin"
                  class="comment-status-middle"
                >
                  <b-button
                    slot="trigger"
                    type="is-text"
                    size="is-small"
                    style="text-decoration: none; color: gray;"
                    aria-controls="replyComment"
                    @click="reply"
                  >
                    <IconText
                      icon="corner-up-left"
                      text="Reply"
                      size="1x"
                      color="gray"
                      text-color="gray"
                    />
                  </b-button>
                </div>
                <div
                  v-if="isLogin"
                  class="comment-status-right"
                >
                  <b-dropdown
                    hoverable
                    aria-role="list"
                  >
                    <b-button
                      slot="trigger"
                      type="is-text"
                      size="is-small"
                      style="text-decoration: none; color: gray;"
                    >
                      <MoreHorizontalIcon size="1x" />
                    </b-button>

                    <b-dropdown-item
                      aria-role="listitem"
                      @click="quoteReply"
                    >
                      Quote reply
                    </b-dropdown-item>
                  </b-dropdown>
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
                v-bind="commentArgs"
                class="reply-comment"
              />
            </b-collapse>

            <!-- sub comments -->
            <BField
              v-if="comment.sub_comments_num !== undefined"
              style="margin-bottom: 0px;"
            >
              <div class="sub-comments">
                <div class="sub-comment-left">
                  <a
                    slot="trigger"
                    class="is-primary sub-comment-link"
                    aria-controls="subComments"
                    @click="toggleSubComments"
                  >{{ subCommentIndicator(comment.sub_comments_num) }}</a>
                </div>
              </div>
            </BField>

            <b-collapse
              :open="showSubComments"
              position="is-bottom"
              aria-id="subComments"
            >
              <div
                :is="subCommentsComponent"
                :id="comment.id"
                class="sub-comment-component"
                v-bind="{articleId: comment.article_id, parentCommentId: comment.id, locationData: subCommentData}"
              />
            </b-collapse>
          </div>
        </div>
      </article>
    </div>
  </li>
</template>

<script>
import MarkdownPreview from './MarkdownPreview'
import NewComment from './NewComment'
import Empty from './Empty'
import SubComments from './SubComments'
import Avatar from "./Avatar"
import DatetimeUtil from "./utils/datetime"
import { EventBus, EVENT_SET_COMMENT_EDITOR_CONTENT, EVENT_SCROLL_TO_COMMENT_EDITOR, EVENT_CLOSE_COMMENT_REPLY_VIEW } from '@/event-bus.js'
import Url from './utils/url'
import { mapGetters } from "vuex";
import { IS_LOGIN, USER_ID } from "@/store/modules/store-types.js";
import { USER } from "@/store/modules/module-names";
import Utils from '@/utils'
import VueScrollTo from 'vue-scrollto'
import IconText from '@/components/controllers/IconText'
import ButtonSpan from '@/components/controllers/ButtonSpan'
import { MoreHorizontalIcon } from 'vue-feather-icons'

export default {
  name: "CommentEntity",
  components: {
    MarkdownPreview,
    Avatar,
    NewComment,
    Empty,
    SubComments,
    IconText,
    ButtonSpan,
    MoreHorizontalIcon
  },
  props: {
    comment: {
      type: Object,
      default: () => { return {} }
    },
    showSeparator: {
      type: Boolean,
      default: false,
    },
    locationData: {
      type: Object,
      default: () => { return {} }
    }
  },
  data() {
    return {
      replyComponent: 'Empty',
      isReply: false,
      isQuoteReply: false,
      subCommentsComponent: 'Empty',
      showSubComments: false,
      showOverlay: false,
      subCommentData: undefined,
    }
  },
  computed: {
    createTime: function () {
      return DatetimeUtil.toTimeAgo(this.comment.create_time)
    },
    commentArgs: function () {
      let args = {
        articleId: this.comment.article_id,
        commentId: this.comment.id
      }
      if (this.isQuoteReply) {
        args['comment'] = `> ${this.comment.comment}`
      }
      return args
    },
    ...mapGetters(USER, {
      isLogin: IS_LOGIN,
      userId: USER_ID,
    })
  },
  mounted() {
    this.listenEvents()
    if (!Utils.isObjEmpty(this.locationData)) {
      this.locateComment()
    }
  },
  beforeDestroy() {
    EventBus.$off(EVENT_CLOSE_COMMENT_REPLY_VIEW)
  },
  methods: {
    locateComment() {
      if (this.locationData.child === undefined) {
        // no sub comments, just focus on target comment
        setTimeout(() => {
          // blink
          Utils.blink(() => {
            this.showOverlay = !this.showOverlay
          }, 800, 4)

          // scroll to target comment
          console.log('scroll to commententity')
          VueScrollTo.scrollTo(this.$refs.comment, 500)
        }, 2000)
        return
      }
      // otherwise focus on the target sub comment 
      this.toggleSubComments()
      // update sub comment data
      this.subCommentData = {
        targetId: this.locationData.targetId,
        child: this.locationData.child,
      }
    },
    listenEvents() {
      const self = this;
      EventBus.$on(EVENT_CLOSE_COMMENT_REPLY_VIEW, function (commentId) {
        console.log(`event-bus: ${EVENT_CLOSE_COMMENT_REPLY_VIEW}`)
        // Close comment reply window after finish commenting
        if (commentId === self.comment.id) {
          self.toggleReplyCommentView()
        }
      })
    },
    quoteReply() {
      this.isQuoteReply = true
      this.toggleReplyCommentView()
    },
    reply() {
      this.isQuoteReply = false
      this.toggleReplyCommentView()
    },
    toggleReplyCommentView() {
      this.isReply = !this.isReply;
      this.replyComponent = this.isReply ? 'NewComment' : 'Empty'
    },
    copyUserInfo() {
      let self = this
      let userInfo = `[@${this.comment.nickname}](${Url.getUrls().user(this.comment.user_id)})`
      this.$copyText(userInfo).then(function (e) {
        self.$getUi().toast.success('User id copied!')
      }, function (e) {
        self.$getUi().toast.success(`failed to copy user id: ${JSON.stringify(e)}`)
      })
    },
    toggleSubComments() {
      this.showSubComments = !this.showSubComments
      this.subCommentsComponent = this.showSubComments ? 'SubComments' : 'Empty'
    },
    subCommentIndicator(num) {
      return this.showSubComments ? 'hide replies' : `${num}  more replies`
    }
  },
}
</script>

<style scoped>
.comment-area {
  padding-top: 0.75rem;
  padding-left: 0.75rem;
}

.blink {
  transition: background 3s;
}

.overlay {
  background-color: #d5c7fc;
  opacity: 0.8;
  border-radius: 0.3rem;
}

.separator {
  margin-top: 0.1rem;
  margin-bottom: 0.1rem;
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

.sub-comment-component {
  margin-left: 15px;
  margin-right: 15px;
}

.comment-status {
  display: flex;
  padding-left: 5px;
  padding-bottom: 0%;
}

.comment-status-left {
  align-items: center;
  margin-left: 0;
}

.comment-status-middle {
  align-items: center;
  margin-left: 20px;
}

.comment-status-right {
  align-items: center;
  margin-left: auto;
  order: 3;
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
}

.comment-content {
  padding: 5px;
}

/* this editor style is use to recover the p element style impacted by Buefy */
.comment-content >>> p {
  margin: 5px;
}
</style>

<style scoped src="@/assets/css/comment_item.css">
</style>