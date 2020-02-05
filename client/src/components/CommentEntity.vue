<template>
  <li>
    <article class="media">
      <figure class="media-left">
        <p class="image is-64x64">
          <Avatar />
        </p>
      </figure>
      <div class="media-content">
        <div class="bubble">
          <BField>
            <div class="comment-header">
              <div
                align="left"
                class="comment-header-left"
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
                    <span class="comment-nickname">{{ comment.nickname }}</span>
                  </b-button>
                  <b-dropdown-item
                    aria-role="listitem"
                    @click="copyUserInfo"
                  >
                    Copy user info
                  </b-dropdown-item>
                </b-dropdown>
                <!-- <span class="comment-nickname">{{ comment.nickname }}</span> -->
                <span class="comment-info">commented {{ createTime }}</span>
              </div>
              <div class="comment-header-right">
                <b-button
                  type="is-text"
                  size="is-small"
                  icon-pack="mdi"
                  icon-left="reply"
                  style="text-decoration: none; color: gray;"
                  @click="quoteReply"
                >
                  Reply
                </b-button>
              </div>
            </div>
          </BField>
          <BField>
            <div
              aligh="left"
              class="comment-content"
            >
              <MarkdownPreview :content="comment.comment" />
            </div>
          </BField>
        </div>
      </div>
    </article>
  </li>
</template>

<script>
import MarkdownPreview from './MarkdownPreview'
import Avatar from "./Avatar"
import DatetimeUtil from "./utils/datetime"
import { EventBus, EVENT_SET_COMMENT_EDITOR_CONTENT, EVENT_SCROLL_TO_COMMENT_EDITOR } from '@/event-bus.js'
import Url from './utils/url'

export default {
  name: "CommentEntity",
  components: {
    MarkdownPreview,
    Avatar,
  },
  props: {
    comment: {
      type: Object,
      default: () => { return {} }
    }
  },
  data() {
    return {}
  },
  computed: {
    createTime: function () {
      return DatetimeUtil.toTimeAgo(this.comment.create_time)
    }
  },
  mounted() {
    console.log(`comments: ${JSON.stringify(this.comment)}`)
  },
  methods: {
    quoteReply() {
      // save backquote to comment editor
      let content = `> ${this.comment.comment}`
      EventBus.$emit(EVENT_SET_COMMENT_EDITOR_CONTENT, content)
      // scroll to comment editor
      EventBus.$emit(EVENT_SCROLL_TO_COMMENT_EDITOR)
    },
    copyUserInfo() {
      let self = this
      let userInfo = `[@${this.comment.nickname}](${Url.getUrls().user(this.comment.user_id)})`
      console.log(`userInfo: ${userInfo}`)
      this.$copyText(userInfo).then(function (e) {
        self.$getUi().toast.success('User id copied!')
      }, function (e) {
        self.$getUi().toast.success(`failed to copy user id: ${JSON.stringify(e)}`)
      })
    }
  },
}
</script>

<style scoped>
.comment-header {
  padding: 5px;
}

.comment-header-left {
  display: flex;
  align-items: center;
  float: left;
}

.comment-header-right {
  float: right;
}

.comment-info {
  display: flex;
  align-items: center;
  color: gray;
  font-size: small;
  padding: 5px;
}

.comment-nickname {
  color: #7957d5;
  font-size: small;
  padding: 5px;
  font-weight: bold;
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