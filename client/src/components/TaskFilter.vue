<template>
  <div>
    <h5 class="text-left">筛选用例</h5>
    <b-input-group prepend="用例集">
      <!-- 用例集 -->
      <b-input-group-prepend>
        <b-dropdown
          :text="caseSet"
          variant="primary"
          slot="append"
        >
          <b-dropdown-item
            v-for="s in caseSetOptions"
            v-bind:key="getItemKey(s)"
            @click="selectedCaseSet"
          >
            {{s}}
          </b-dropdown-item>
        </b-dropdown>
      </b-input-group-prepend>

      <!-- 用例名称 -->
      <b-form-input
        type="text"
        placeholder="请输入用例名称，多个用例请使用逗号作为分隔符"
        v-model="caseId"
        @keyup.enter.native="filterCases"
      ></b-form-input>
      <b-input-group-append>
        <b-btn
          variant="primary"
          @click="filterCases"
        >筛选</b-btn>
      </b-input-group-append>
    </b-input-group>
    <br />

    <!-- 状态筛选 -->
    <b-form-radio-group
      v-model="selected"
      :options="options"
      @change="statusChanged"
    ></b-form-radio-group>
  </div>
</template>
<script>
  import Utils from '@/utils.js'

  export default {
    name: "TaskFilter",
    props: {
      caseSets: {}  // 用[]过不了Vue类型校验会报错，原因不明
    },
    data() {
      return {
        caseSetOptions: [],
        caseSet: '全部',
        caseId: '',
        status: String,
        options: [
          { text: '全部用例', value: 'All' },
          { text: '失败用例', value: 'Failed' },
          { text: '成功用例', value: 'Success' },
          { text: '排队用例', value: 'NotRun' },
        ],
        selected: 'All'
      }
    },
    mounted() {

    },
    watch: {
      caseSets: function (newVal, oldVal) {
        this.caseSetOptions = ["全部"];
        this.caseSetOptions.push(...this.caseSets);
        console.log("caseSets: " + this.caseSets);
      }
    },
    methods: {
      getItemKey(item) {
        // 只能放方法中写，不然无法识别 Utils 类
        return Utils.getObjectHash(item)
      },
      filterCases() {
        console.log("filterCase...");
        var args = {
          status: this.selected,
        };
        if (this.caseSet !== '全部') {
          args.caseSet = this.caseSet;
        }
        if (this.caseId !== '') {
          args.caseId = this.caseId;
        }

        this.emitFilterConditionChangedEvent(args);
      },
      emitFilterConditionChangedEvent(args) {
        this.$emit('filterConditionChanged', args);
      },
      selectedCaseSet(item) {
        this.caseSet = item.currentTarget.innerText;
        console.log("caseSetItemChanged: " + this.caseSet);
      },
      statusChanged(value) {
        this.caseSet = '全部';
        this.selected = value;
        let args = {
          status: this.selected
        };
        this.emitFilterConditionChangedEvent(args);
      }
    }
  }
</script>

