<template>
  <!-- class="avatar-size border" -->
  <Gravatar
    :hash="idVal"
    class="border"
  />
</template>

<script>
import Gravatar from 'vue-gravatar';
import { mapGetters } from 'vuex'
import { USER_ID } from '@/store/modules/store-types.js'
import { USER } from '@/store/modules/module-names'
import md5 from 'md5'

export default {
  name: 'Avatar',
  components: {
    Gravatar,
  },
  props: {
  },
  data() {
    return {
      idVal: ''
    }
  },
  computed: {
    ...mapGetters(USER, {
      userId: USER_ID
    }),

  },
  mounted() {
    this.idVal = md5(this.userId)
    console.log(`avatar mounted: ${this.userId}, md5:${this.idVal}`)
  },
  methods: {
  },
}
</script>

<style scoped>
.border {
  border-radius: 5px;
  border: #e7eaed solid 1px;
  padding: 2px;
}
</style>