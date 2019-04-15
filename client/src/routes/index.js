import Vue from 'vue';
import Router from 'vue-router';
// import ResultDiff from '@/components/ResultDiff';
import NotFound from '@/components/NotFound';
import Home from '@/components/Home';
import Articles from '@/components/Articles';
import About from '@/components/About';

Vue.use(Router);

export default new Router({
  routes: [{
      path: '/',
      name: 'Home',
      component: Home
    },
    {
      path: '/articles',
      name: 'Articles',
      component: Articles
    },
    {
      path: '/about',
      name: 'About',
      component: About
    },
    {
      path: '*',
      name: 'NotFound',
      component: NotFound
    }
  ]
});