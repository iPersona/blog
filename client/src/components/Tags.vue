<template>

  <div v-if="isLogin && isAdmin">
    <b-taglist>
      <b-tag
        type="is-primary"
        size="is-medium"
        closable
        aria-close-label="Close tag"
        v-for="tag in tags"
        v-bind="tag.uuid"
        v-if="tag.isActive"
        @close="tag.isActive = false; deletedTags.push(tag);"
      >
        {{tag.tag}}
      </b-tag>
    </b-taglist>
  </div>
  <div v-else>

  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import { IS_LOGIN, IS_ADMIN } from '@/store-types.js'
import Api from '@/api'
import Ui from './utils/ui'
import Log from './utils/log'

export default {
  name: 'Tags',
  data() {
    return {
      tags: [],
      deletedTags: [],
      log: new Log(this),
      ui: new Ui(this),
    }
  },
  components: {

  },
  computed: {
    ...mapGetters({
      isLogin: IS_LOGIN,
      isAdmin: IS_ADMIN,
    }),
  },
  async mounted() {
    this.getTags()
  },
  methods: {
    async getTags() {
      let api = new Api()
      let rsp = await api.getTags();
      this.log.debug(`tags: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        return
      }
      let tags = rsp.data
      this.tags = this.addStatus(tags)
    },
    addStatus(tags) {
      for (var i in tags) {
        tags[i]['isActive'] = true
      }
      return tags
    },
  },
}
</script>