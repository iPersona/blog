import Vue from 'vue';

// Events
export const EVENT_RELOAD_ARTICLE = 'reload-article'
export const EVENT_RELOAD_ARTICLE_LIST = 'reload-article-list'
export const EVENT_HIDE_HEADER = 'hide-header'
export const EVENT_SHOW_HEADER = 'show-header'
export const EVENT_MARKDOWN_EDITOR_CONTENT_READY = 'markdown-editor-content-ready'
export const EVENT_ARTICLE_EDITOR_CLOSED = 'article-editor-closed'
export const EVENT_RELOAD_COMMENTS = "reload-comments"

// Bus
export const EventBus = new Vue();