<template>
  <div class="container-fluid">
    <div class="row">
      <div class="col">
        <task-list
          ref="taskList"
          :filterCondition="filterCondition"
          @selectedItemChanged="selectedItemChanged"
          @dataChanged="dataChanged"
        />
      </div>
      <div class="col-9">
        <div class="sticky-top pt-5">
          <!-- 过滤器 -->
          <task-filter
            :caseSets="caseSets"
            @filterConditionChanged="filterConditionChanged"
          />
          <hr />
          <!-- 控制器 -->
          <task-control ref="taskControl"></task-control>
          <!-- 任务进度 -->
          <task-progress ref="taskProgress" />
          <!-- 任务信息 -->
          <task-info ref="taskInfo" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  import TaskList from './TaskList'
  import TaskInfo from './TaskInfo'
  import Api from '@/api.js'
  import Utils from '@/utils.js'
  import TaskProgress from './TaskProgress'
  import TaskFilter from './TaskFilter'
  import TaskControl from './TaskControl'


  export default {
    name: 'Debug',
    components: {
      TaskList,
      TaskInfo,
      TaskProgress,
      TaskFilter,
      TaskControl
    },
    data() {
      return {
        caseSets: [],
        filterCondition: {},
      }
    },
    mounted() {
    },
    methods: {
      selectedItemChanged(args) {
        console.log("selectedItemChanged: " + JSON.stringify(args));
        this.$refs.taskInfo.updateData(args); // 更新任务信息组件
        this.$refs.taskControl.updateCurrentCase(args); // 更新任务控制组件
      },
      dataChanged(caseFile) {
        let args = {
          max: caseFile.cases.length,
          success: caseFile.cases.filter(item => item.status === "Success").length,
          failed: caseFile.cases.filter(item => item.status === "Failed").length,
          pending: caseFile.cases.filter(item => item.status === "NotRun").length,
          isRunning: caseFile.isRunning,
        }
        console.log("dataChanged: " + JSON.stringify(args));
        this.$refs.taskProgress.updateProgress(args);
        let status = {
          isRunning: caseFile.isRunning,
          cases: this.compactCases(caseFile.cases),
          taskType: caseFile.taskType,
        };
        this.$refs.taskControl.updateStatus(status);
        this.caseSets = this.getCaseSets(caseFile.cases);
      },
      getCaseSets(caseFile) {
        // return [... new Set(caseFile.map(d => d.caseSet))]
        return caseFile.map(d => d.caseSet).filter((value, index, self) => {
          return self.indexOf(value) === index
        });
      },
      filterConditionChanged(args) {
        console.log("filterConditionChanged: " + JSON.stringify(args));
        this.filterCondition = args;
        let caseSets = args.caseSets === undefined ? this.caseSets : [args.caseSet];
        let cases = Utils.getCaseIdFromQueryString(args.caseId);
        let filterCases = caseSets.map(v => {
          return {
            caseSet: v,
            cases: cases
          };
        });
        this.$refs.taskControl.updateFilterCases(filterCases); // 更新任务控制组件
      },
      compactCases(cases) {
        let caseSets = this.getCaseSets(cases);
        let compactVal = []
        for (var i = 0; i < caseSets.length; i++) {
          let val = {
            caseSet: caseSets[i],
            cases: cases.filter(v => v.caseSet === caseSets[i])
          }
          compactVal.push(val);
        }
        return compactVal;
      }
    }
  }
</script>

