<template>
  <div>
    <h1>
      Articles with tag: {{ preTagName }}
    </h1>
    <ArticleList :is-filtered-by-tag="true" />
  </div>
</template>

<script>
import ArticleList from './ArticleList'

import { mapGetters } from 'vuex'
import { mapMutations } from 'vuex'
import { SAVE_TAG } from '@/store/modules/mutation-types.js'
import { TAG } from '@/store/modules/module-names'
import { TAG_ID, TAG_NAME } from '@/store/modules/store-types'

export default {
  name: 'ArticleListByTag',
  components: {
    ArticleList,
  },
  props: {
    tagId: undefined,
    tagName: undefined,
  },
  data() {
    return {
    }
  },
  computed: {
    ...mapGetters(TAG, {
      preTagId: TAG_ID,
      preTagName: TAG_NAME,
    })
  },
  async mounted() {
    this.$getLog().debug(`xxxxxxxx tagId: ${this.tagId}, tagName: ${this.tagName}`)
    if (this.tagId !== undefined && this.tagName !== undefined) {
      let tag = {
        id: this.tagId,
        name: this.tagName,
      }
      this.$getLog().debug(`yyyy tag: ${JSON.stringify(tag)}`)
      await this.saveTag(tag)
    }
  },
  methods: {
    ...mapMutations(TAG, {
      saveTag: SAVE_TAG
    }),
  },
}
</script>