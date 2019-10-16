<template>
  <section class="container">
    <!-- search key -->
    <b-field grouped>
      <b-input
        type="search"
        v-model="searchKey"
        placeholder="Search..."
        expanded
        icon-pack="fas"
        icon="search"
        @input="searchTag"
      />
    </b-field>
    <b-field
      grouped
      group-multiline
    >
      <div
        class="control"
        v-for="tag in filteredTags"
        v-bind:key="tag.id"
      >
        <router-link :to="{name: 'articles_with_tag', params: {tagId: tag.id, tagName: tag.tag}}">
          <b-taglist attached>
            <b-tag
              type="is-light"
              size="is-medium"
            >{{tag.tag}}</b-tag>
            <b-tag
              type="is-primary"
              size="is-medium"
            >{{tag.count}}</b-tag>
          </b-taglist>
        </router-link>
      </div>
    </b-field>
  </section>
</template>

<script>
import Api from '@/api'
import Log from '@/components/utils/log'

export default {
  name: 'TagsManagement',
  components: {},
  data() {
    return {
      tags: [],
      searchKey: '',
      filteredTags: [],
    }
  },
  watch: {
    searchKey(newVal, oldVal) {
      if (newVal === '') {  // reset filtered tags if key is empty
        this.filteredTags = this.tags;
      }
    }
  },
  async mounted() {
    this.reloadTags()
  },
  methods: {
    async reloadTags() {
      this.tags = await this.getTags()
      this.filteredTags = this.tags
      this.$log.debug(`filteredTags: ${JSON.stringify(this.filteredTags)}`)
    },
    async getTags() {
      let api = new Api()
      let rsp = await api.getTagsWithCount();
      this.$log.debug(`tags: ${JSON.stringify(rsp)}`)
      if (!Api.isSuccessResponse(rsp)) {
        return
      }
      return rsp.data
    },
    searchTag() {
      if (this.searchKey === '') {
        return
      }
      this.filteredTags = this.tags.filter(item => {
        let t = item.tag.toString().toLowerCase()
        let k = this.searchKey.toString().toLowerCase()
        return t.indexOf(k) >= 0
      })
      this.$log.debug(`this.filtredTags: ${JSON.stringify(this.filteredTags)}`)
    },
  },
}
</script>