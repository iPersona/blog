<template>
  <div>
    <h1>测试结果</h1>
    <!-- <case :caseObj="caseData"></case> -->
    <case
      v-for="caseObj in caseData"
      v-bind:key="getCaseKey(caseObj)"
      :caseObj="caseObj"
      :collapseId="getCaseKey(caseObj)"
    ></case>
  </div>
</template>

<script>
  import Case from './Case'
  import Api from '@/api.js'

  export default {
    name: 'Result',
    props: {},
    components: {
      Case
    },
    data() {
      return {
        caseData: [{
          result: '{\r\n"a": 1,\r\n "b": "01020304"\r\n}',
          expect: '{\r\n"a": 1,\r\n "b": "01020404"\r\n}',
          caseSet: 'PBOC测试案例',
          caseName: 'case-001',
          status: 'Fail',
        },
        {
          result: '{\r\n"b": "01020304",\r\n "a": 1\r\n}',
          expect: '{\r\n"a": 1,\r\n"b": "01020404"\r\n}',
          caseSet: 'PBOC测试案例',
          caseName: 'case-002',
          status: 'Fail',
        },
        {
          result: '{\r\n"中文b": "01020304",\r\n "a": 1\r\n}',
          expect: '{\r\n"中文b": "01020304",\r\n "a": 1\r\n}',
          caseSet: 'PBOC测试案例',
          caseName: 'case-003',
          status: 'Success',
        }]
      }
    },
    mounted: function () {
      //this.init()
    },
    methods: {
      async init() {
        let api = new Api();
        await api.getResults();
      },
      getCaseKey(caseObj) {
        let hash = require('object-hash');
        let collapseId = 'cid-' + hash(caseObj);
        console.log("collapseId: " + collapseId);
        return collapseId;
      }
    }
  }
</script>
  
  
