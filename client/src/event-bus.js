import Vue from 'vue'

// Events
export const EVENT_RELOAD_ARTICLE = 'reload-article'
export const EVENT_RELOAD_ARTICLE_LIST = 'reload-article-list'
export const EVENT_MARKDOWN_EDITOR_CONTENT_READY = 'markdown-editor-content-ready'
export const EVENT_ARTICLE_EDITOR_CLOSED = 'article-editor-closed'
export const EVENT_RELOAD_COMMENTS = 'reload-comments'
export const EVENT_STATISTIC_DATE_RANGE_CHANGED = 'statistic-data-range-changed'
export const EVENT_STATISTIC_VIEW_MOUNTED = 'statistic-view-mounted'
export const EVENT_SET_COMMENT_EDITOR_CONTENT = 'set-comment-editor-content'
export const EVENT_SCROLL_TO_COMMENT_EDITOR = 'scroll-to-comment-editor'
export const EVENT_CLOSE_COMMENT_REPLY_VIEW = 'toggle-reply-comment-view'

// Bus
export const EventBus = new Vue()