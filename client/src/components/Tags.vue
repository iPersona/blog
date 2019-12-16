<template>
  <section class="container">
    <!-- search key -->
    <BField grouped>
      <BInput
        v-model="searchKey"
        type="search"
        placeholder="Search..."
        expanded
        icon-pack="fas"
        icon="search"
        @input="searchTag"
      />
    </BField>
    <BField
      grouped
      group-multiline
    >
      <div
        v-for="tag in filteredTags"
        :key="tag.id"
        class="control"
      >
        <RouterLink :to="{name: 'articles_with_tag', params: {tagId: tag.id, tagName: tag.tag}}">
          <BTaglist attached>
            <BTag
              type="is-light"
              size="is-medium"
            >
              {{ tag.tag }}
            </BTag>
            <BTag
              type="is-primary"
              size="is-medium"
            >
              {{ tag.count }}
            </BTag>
          </BTaglist>
        </RouterLink>
      </div>
    </BField>
    <br>
  </section>
</template>

<script>
import Api from '@/api'

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
      this.$getLog().debug(`filteredTags: ${JSON.stringify(this.filteredTags)}`)
    },
    async getTags() {
      let api = new Api()
      let rsp = await api.getTagsWithCount();
      this.$getLog().debug(`tags: ${JSON.stringify(rsp)}`)
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
      this.$getLog().debug(`this.filtredTags: ${JSON.stringify(this.filteredTags)}`)
    },
  },
}
</script>