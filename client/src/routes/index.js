import Vue from 'vue'
import Router from 'vue-router'
import store from '@/store/index'
import {
    TAG_ID,
    TAG_NAME
} from '@/store/modules/store-types'
import {
    TAG
} from '@/store/modules/module-names'

// import ResultDiff from '@/components/ResultDiff'
import NotFound from '@/components/NotFound'
import Home from '@/components/Home'
import Articles from '@/components/ArticleList'
import Article from '@/components/Article'
import About from '@/components/About'
import Login from '@/components/Login'
import Tags from '@/components/Tags'
import ArticleEditor from '@/components/ArticleEditor'
import Settings from '@/components/Settings'
import Management from '@/components/management/Management'
import Statistic from '@/components/management/Statistic'
import ArticlesManagement from '@/components/management/Articles'
import TagsManagement from '@/components/management/Tags'
import ArticleListByTag from '@/components/ArticleListByTag'

Vue.use(Router)

export default new Router({
    routes: [{
            path: '/',
            name: 'home',
            component: Home
        },
        {
            path: '/articles',
            name: 'articles',
            component: Articles,
        },
        {
            path: '/articles/tag',
            name: 'articles_with_tag',
            component: ArticleListByTag,
            props: true,
        },
        {
            path: '/tags',
            name: 'tags',
            component: Tags
        },
        {
            path: '/article/:id',
            name: 'article',
            component: Article
        },
        {
            path: '/article',
            name: 'new_post',
            component: ArticleEditor
        },
        {
            path: '/about',
            name: 'about',
            component: About
        },
        {
            path: '/login',
            name: 'login',
            component: Login
        },
        {
            path: '/management',
            name: 'management',
            component: Management,
            children: [{
                    path: 'stat',
                    name: 'stat',
                    component: Statistic,
                },
                {
                    path: 'settings',
                    name: 'management-settings',
                    component: Settings
                },
                {
                    path: 'tags',
                    name: 'tagsManagement',
                    component: TagsManagement
                },
                {
                    path: 'articles',
                    name: 'articlesManagement',
                    component: ArticlesManagement
                }
            ]
        },
        {
            path: '/settings',
            name: 'settings',
            component: Settings
        },
        {
            path: '*',
            name: 'NotFound',
            component: NotFound
        }
    ]
})