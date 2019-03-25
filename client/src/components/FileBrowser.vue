<template>
  <div>
    <!-- 过滤器 -->
    <div class="navigation-filter d-flex mx-3">
      <b-form-input
        id="filter"
        v-model="treeFilter"
        placeholder="Type to filter..."
      >
      </b-form-input>
    </div>
    <!-- 文件浏览器 -->
    <div>
      <tree
        :data="treeData"
        :options="treeOptions"
        :filter="treeFilter"
        v-model="selectedNode"
        ref="tree"
      >
        <span
          class="tree-text"
          slot-scope="{ node }"
        >
          <div v-if="isDir(node)">
            <div v-if="isOpen">
              <!-- 文件夹打开 -->
              <ios-folder-open-icon />
              {{ node.text }}
            </div>
            <div v-else>
              <!-- 文件夹关闭 -->
              <ios-folder-icon />
              {{ node.text }}
            </div>
          </div>
          <div v-else>
            <div v-if="isJson(node)">
              <logo-javascript-icon />
              {{ node.text }}
            </div>
            <div v-else>
              <ios-document-icon />
              {{ node.text }}
            </div>
          </div>
        </span>
      </tree>
    </div>
  </div>
</template>

<script>
  import LiquorTree from 'liquor-tree'
  import IosFolderIcon from 'vue-ionicons/dist/ios-folder.vue'
  import IosFolderOpenIcon from 'vue-ionicons/dist/ios-folder-open.vue'
  import LogoJavascriptIcon from 'vue-ionicons/dist/logo-javascript.vue'
  import IosDocumentIcon from 'vue-ionicons/dist/ios-document.vue'
  import Api from '@/api.js'

  export default {
    name: 'FileBrowser',
    components: {
      IosFolderIcon,
      IosFolderOpenIcon,
      LogoJavascriptIcon,
      IosDocumentIcon,
      [LiquorTree.name]: LiquorTree
    },
    data() {
      return {
        treeFilter: '',
        treeOptions: {
          multiple: false,
          filter: {
            plainList: true
          },
          /* 读取文件列表 */
          async fetchData() {
            // 只能通过 fetchData 属性配置的方式异步更新数据
            let api = new Api()
            let data = await api.getList()
            console.log("treeData:" + JSON.stringify(data));
            return data
          }
        },
        selectedNode: null,
        selectedFilePath: '',
        treeData: '',/* [
          {
            text: 'cases', children: [
              {
                text: 'PBOC测试案例', children: [
                  { text: 'case-001.json' },
                  { text: 'case-002.json' },
                  { text: 'case-003.json' }
                ]
              },
              {
                text: 'QPBOC测试案例', children: [
                  { text: 'case-001.json' },
                  { text: 'case-002.json' },
                  { text: 'case-003.json' }
                ]
              }
            ]
          },
          {
            text: 'cfg',
            children: [
              { text: 'case.json' },
              { text: 'CaseConfig.json' },
            ]
          },
          {
            text: 'CaseConfig.json'
          },
          {
            text: 'CommunicatConfig.ini'
          },
          {
            text: 'Data.json'
          }
        ] */
      }
    },
    mounted() {
      // 监听节点选中事件
      this.$refs.tree.$on('node:selected', this.selectedNodeEvent);
    },
    methods: {
      isJson(node) {
        console.log("text: " + node.text);
        return !node.hasChildren()
          && node.text.split('.').pop().toLowerCase() === 'json'
      },
      isOpen(node) {
        return node.expanded()
      },
      isDir(node) {
        return node.hasChildren()
      },
      selectedNodeEvent(node, newNode) {
        if (this.isDir(node)) {
          // 忽略目录
          return;
        }

        console.log("node: " + JSON.stringify(node));
        // console.log("newNode: " + JSON.stringify(newNode));
        let args = {
          path: this.getFullPath(node)
        }
        console.log("path: " + args.path);
        this.$emit('selectedNodeChanged', args);
      },
      getFullPath(node) {
        console.log("getFullPath-node: " + JSON.stringify(node));
        const fullPath = [node.text]
        node.recurseUp(parentEl => fullPath.unshift(parentEl.text))
        return fullPath.join('/')
      }
    }
  }
</script>