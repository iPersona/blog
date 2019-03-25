<template>
  <div role="tablist">
    <b-card
      no-body
      class="mb-1"
    >
      <b-card-header
        header-tag="header"
        class="p-1"
        role="tab"
      >
        <b-btn
          block
          href="#"
          v-b-toggle="collapseId"
          :variant="variant"
        >{{caseSet}} / {{caseName}}</b-btn>
      </b-card-header>
      <b-collapse
        :id="collapseId"
        visible
        accordion="my-accordion"
        role="tabpanel"
        @show="show"
      >
        <b-card-body>
          <!-- 用例描述 -->
          <div>
            <h4 class="text-left">用例描述</h4>
            <p class="text-left">{{ description }}</p>
          </div>
          <hr>
          <div>
            <h4 class="text-left">测试结果</h4>
            <!-- diff 视图 -->
            <div v-if="!success">
              <div class="row">
                <div class="col mr-0 pr-0">
                  <b-btn
                    block
                    variant="outline-success"
                  >
                    预期
                  </b-btn>
                </div>
                <div class="col ml-0 pl-0">
                  <b-btn
                    block
                    variant="outline-danger"
                  >
                    实际
                  </b-btn>
                </div>
              </div>
              <div class="row">
                <div class="col">
                  <result-diff
                    :exp="expect"
                    :act="result"
                  ></result-diff>
                </div>
              </div>
            </div>

            <!-- 结果视图 -->
            <div
              v-if="success"
              class="d-flex text-left"
            >
              <!-- <tree-view
              :caseFile="JSON.parse(result)"
              :options="jsonTreeOpt"
            ></tree-view> -->
              <vue-json-pretty
                :deep="4"
                :path="'res'"
                :caseFile="JSON.parse(result)"
              >
              </vue-json-pretty>
            </div>

          </div>
        </b-card-body>
      </b-collapse>
    </b-card>
  </div>
</template>

<script>
  import ResultDiff from './ResultDiff'
  import VueJsonPretty from 'vue-json-pretty'
  import Api from '@/api.js'

  export default {
    name: 'Case',
    props: {
      caseObj: Object,
      collapseId: '',
    },
    components: {
      ResultDiff,
      VueJsonPretty
    },
    data() {
      return {
        caseSet: '',
        caseName: '',
        result: '{}', // [必须是合法的JSON字符串，否则会造成JSON.parse()解析出错](https://stackoverflow.com/a/42700657/10366378)
        expect: '{}',
        status: '',
        variant: "outline-success",
        success: true,
        jsonTreeOpt: {
          maxDepth: 3,
          rootObjectKey: "Result"
        },
        description: '',
      }
    },
    mounted: function () {
      this.init();
    },
    methods: {
      init() {
        console.log('aaaaaaaaaaa');

        this.caseSet = this.caseObj['caseSet'];
        this.caseName = this.caseObj['caseName'];
        this.result = this.caseObj['result'];
        this.expect = this.caseObj['expect'];
        this.status = this.caseObj['status'];

        this.variant = "outline-danger";
        this.success = false;
        if (this.status === 'Success') {
          this.variant = "outline-success";
          this.success = true;
        }

        console.log("result: " + this.result);
        let json = JSON.parse(this.result);
        console.log("json: " + JSON.stringify(json));
      },
      async show() {
        try {
          console.log("show");
          let api = new Api();
          let rsp = await api.getCaseDescription(this.caseSet, this.caseName);
          this.description = rsp;
          console.log("rsp-description: " + rsp);
        } catch (error) {
          console.error("getCaseDescription failed: " + error.message);
          this.description = "无法加载用例描述：" + error.message;
        }
      }
    }
  }
</script>

<style>
  .top {
    margin-bottom: 10px;
  }
  .block {
    float: left;
    padding: 0 15px;
    width: 50%;
    box-sizing: border-box;
  }
</style>
