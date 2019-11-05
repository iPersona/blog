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
            <div align="left">
              <span class="comment-nickname">{{ comment.nickname }}</span>
              <span class="comment-info">•</span>
              <span class="comment-info">{{ createTime }}</span>
            </div>
          </BField>
          <BField>
            <div class="comment-content">
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

  },
}
</script>

<style scoped>
.comment-info {
  color: gray;
  font-size: small;
  padding: 5px;
}

.comment-nickname {
  color: #7957d5;
  font-size: small;
  padding: 5px;
}

.comment-content {
  padding: 5px;
}
</style>

<style scoped src="@/assets/css/comment_item.css">
</style>