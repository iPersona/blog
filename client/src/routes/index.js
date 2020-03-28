import Vue from 'vue'
import Router from 'vue-router'

import NotFound from '@/components/NotFound'
import Home from '@/components/Home'
import Articles from '@/components/ArticleList'
import Article from '@/components/Article'
import About from '@/components/About'
import Login from '@/components/Login'
import Tags from '@/components/Tags'
import ArticleEditor from '@/components/ArticleEditor'
import Settings from '@/components/settings/Settings'
import Management from '@/components/management/Management'
import Statistic from '@/components/management/Statistic'
import ArticlesManagement from '@/components/management/Articles'
import TagsManagement from '@/components/management/Tags'
import ArticleListByTag from '@/components/ArticleListByTag'
import Profile from '@/components/settings/Profile'
import PersonalSettings from '@/components/settings/PersonalSettings'
import Security from '@/components/settings/Security'
import ErrorPage from '@/components/ErrorPage'
import Verify from '@/components/EmailVerify'
import User from '@/components/User'
import CommentNotification from '@/components/notification/CommentNotification'
import Notifications from '@/components/notification/Notifications'

Vue.use(Router)

export default new Router({
    linkActiveClass: 'is-active',
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
            path: '/user/:id',
            name: 'user',
            component: User
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
            path: '/verify/:token',
            name: 'verify',
            component: Verify
        },
        {
            path: '/notification',
            name: 'notification',
            component: Notifications,
            redirect: '/notification/comment',
            children: [{
                path: 'comment',
                name: 'comment-notification',
                component: CommentNotification
            }]
        },
        {
            path: '/management',
            name: 'management',
            component: Management,
            redirect: '/management/stat',
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
            path: '/personal',
            name: 'personal',
            component: PersonalSettings,
            redirect: '/personal/profile',
            children: [{
                    path: 'profile',
                    name: 'profile',
                    component: Profile,
                },
                {
                    path: 'security',
                    name: 'security',
                    component: Security,
                },
            ]
        },
        {
            path: '/error',
            name: 'error',
            component: ErrorPage,
        },
        {
            path: '*',
            name: 'NotFound',
            component: NotFound
        }
    ]
})