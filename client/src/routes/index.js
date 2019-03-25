import Vue from 'vue';
import Router from 'vue-router';
// import ResultDiff from '@/components/ResultDiff';
import NotFound from '@/components/NotFound';
import Result from '@/components/Result';
import CaseEditor from '@/components/CaseEditor';
import Home from '@/components/Home';
import Operation from '@/components/Operation';

Vue.use(Router);

export default new Router({
  routes: [{
      path: '/',
      name: 'Home',
      component: Home
    },
    {
      path: '/result',
      name: 'Result',
      component: Result
    },
    {
      path: '/files',
      name: "CaseEditor",
      component: CaseEditor
    },
    {
      path: '/oper',
      name: 'Operation',
      component: Operation
    },
    {
      path: '*',
      name: 'NotFound',
      component: NotFound
    }
  ]
});