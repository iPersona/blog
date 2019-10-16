import qs from 'qs';
import Vue from 'vue';
import store from '@/store/index'
import {
    STORE_KEY
} from '@/store/modules/user'
import {
    TOKEN
} from '@/store/modules/store-types'
import {
    LOGOUT
} from '@/store/modules/mutation-types'
import {
    USER
} from '@/store/modules/module-names'

export default class Api {
    constructor(vue) {
        this.axios = Vue.axios;
        // this.host = document.location.host;
        this.host = 'http://localhost:8880/api/v1';
        console.log('host: ' + this.host);
        this.vue = vue;
        (this.config = {
            Accept: 'application/json',
            'Content-Type': 'application/x-www-form-urlencoded',
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Credentials': true
        }),
        (this.url = {
            login: `${this.host}/user/login`, // 登录
            createArticle: `${this.host}/article/new`, // 创建文章
            deleteArticle: `${this.host}/article/delete`, // 删除文章
            adminView: `${this.host}/article/admin/view`, // 获取文章html
            adminViewRaw: `${this.host}/article/admin/view_raw`, // 获取文章md
            adminViewAll: `${this.host}/article/admin/view_all`, // 获取文章列表
            editArticle: `${this.host}/article/edit`, // 编辑文章
            publishArticle: `${this.host}/article/publish`, // 发布文章
            visitorViewAll: `${this.host}/articles`, // 游客：访问文章列表
            visitorViewArticle: `${this.host}/article/view`, // 游客：访问文章列表
            articleNumber: `${this.host}/article/count`, // 访客：文章数量
            articleNumberByTag: `${this.host}/article/tag/count`,
            articleByTag: `${this.host}/article/tag`,
            signup: `${this.host}/user/new`, // 游客：用户注册
            userExist: `${this.host}/user/exist`, // 游客：检查用户是否存在
            getTags: `${this.host}/tag/view`,
            getTagsWithCount: `${this.host}/tag/view/count`,
            addTags: `${this.host}/tag/new`,
            delTag: `${this.host}/tag/delete`,
            editTag: `${this.host}/tag/edit`,
            updateTags: `${this.host}/tag/update`,
        });
        // request拦截器
        this.axios.interceptors.request.use(req => {
            // Do something before request is sent
            // console.log(`store: ${store}`)
            // console.log(`STORE_KEY: ${STORE_KEY}`)
            if (localStorage[STORE_KEY] !== undefined) {
                // req.headers['X-Token'] =
                //     localStorage // 让每个请求携带token--['X-Token']为自定义key
                // 请根据实际情况自行修改
                req.headers['Authorization'] = store.getters[`user/${TOKEN}`];
                // console.log(`token: ${store.getters[TOKEN]}`)
            }
            return req
        }, error => {
            // Do something with request error
            console.log(error) // for debug
            Promise.reject(error)
        });
    }

    async getArticleNumberByTag(tagId) {
        return this.get(this.url.articleNumberByTag, {
            tag_id: tagId
        })
    }

    async getArticlesByTag(tagId, limit, offset) {
        return this.get(this.url.articleByTag, {
            tag_id: tagId,
            limit: limit,
            offset: offset,
        })
    }

    async getTagsWithCount() {
        return this.get(this.url.getTagsWithCount)
    }

    async updateTags(modifiedTags, addedTags, deletedTags) {
        let args = {}
        if (modifiedTags !== undefined) {
            args.modified_tags = modifiedTags
        }
        if (addedTags !== undefined) {
            args.added_tags = addedTags
        }
        if (deletedTags !== undefined) {
            args.deleted_tags = deletedTags
        }
        return this.post(this.url.updateTags, args)
    }

    async editTag(tagObj) {
        return this.post(this.url.editTag, tagObj)
    }

    async delTag(id) {
        return this.delete(this.url.delTag, id)
    }

    async addTags(tags) {
        let data = tags.map(item => {
            return {
                "tag": item
            }
        })
        return this.post(this.url.addTags, data)
    }

    async getTags() {
        return this.get(this.url.getTags)
    }

    async signup(info) {
        return this.post(this.url.signup, {
            account: info.username,
            password: info.password,
            nickname: info.nickname,
            say: info.sign,
            email: info.email
        });
    }

    async login(account, password, remember) {
        return this.post(
            this.url.login, {
                account: account,
                password: password,
                remember: remember
            });
    }

    async createArticle(title, rawContent, existTags, newTags, publish) {
        let args = {
            title: title,
            raw_content: rawContent
        };
        if (existTags !== undefined) {
            args.exist_tags = existTags;
        }
        if (newTags !== undefined) {
            args.new_tags = newTags;
        }
        args.publish = publish;
        return this.post(this.url.createArticle, args);
    }

    async checkUserExist(email) {
        return this.post(this.url.userExist, {
            email: email
        });
    }

    async deleteArticle(id) {
        return this.post(this.url.deleteArticle, {
            id: id
        });
    }

    async adminView(id) {
        return this.get(this.url.adminView, {
            id: id
        });
    }

    async adminViewRawArticle(id) {
        return this.get(this.url.adminViewRaw, {
            id: id
        });
    }

    async adminViewAll(limit, offset) {
        return this.get(this.url.adminViewAll, {
            limit: limit,
            offset: offset
        });
    }

    async editArticle(
        id, title, rawContent, newChoiceAlreadyExistsTags, deselectTags,
        newTags) {
        let args = {
            id: id,
            title: title,
            raw_content: rawContent,
        };
        if (newChoiceAlreadyExistsTags !== undefined) {
            args.new_choice_already_exists_tags = newChoiceAlreadyExistsTags;
        }
        if (deselectTags !== undefined) {
            args.deselect_tags = deselectTags;
        }
        if (newTags !== undefined) {
            args.new_tags = newTags;
        }
        return this.post(this.url.editArticle, args);
    }

    async publishArticle(id) {
        return this.post(this.url.publishArticle, {
            id: id,
            publish: true,
        });
    }

    async visitorViewAll(limit, offset) {
        return this.get(this.url.visitorViewAll, {
            limit: limit,
            offset: offset,
        });
    }

    async visitorViewArticle(id) {
        let url = `${this.url.visitorViewArticle}/${id}`;
        return this.get(url);
    }

    async getArticleNumber() {
        return this.get(this.url.articleNumber);
    }

    async getResults() {
        return this.get(this.url.results);
    }

    async getList() {
        return this.get(this.url.list);
    }

    async getFile(path) {
        return this.get(this.url.file, {
            path: path
        });
    }

    async saveFile(path, content) {
        return this.post(this.url.save, {
            path: path,
            content: content
        });
    }

    async getCaseDescription(caseSet, caseId) {
        return this.get(this.url.description, {
            set: caseSet,
            id: caseId
        });
    }

    async getTask() {
        return this.get(this.url.task);
    }

    async getTaskInfo(caseSet, caseId) {
        return this.get(this.url.taskinfo, {
            set: caseSet,
            id: caseId
        });
    }

    async controlTask(start, taskType, cases) {
        if (cases === undefined) {
            return this.post(this.url.control, {
                start: start,
                taskType: taskType
            });
        }

        console.log('controlTask: cases: ' + JSON.stringify(cases));
        return this.post(
            this.url.control, {
                start: start,
                taskType: taskType,
                cases: JSON.stringify(cases)
            });
    }

    async reloadCases() {
        return this.post(this.url.reloadcases);
    }

    async post(url, args) {
        return this.doRequest(url, 'post', args);
    }

    async get(url, args) {
        return this.doRequest(url, 'get', args);
    }

    async delete(url, args) {
        return this.doRequest(url, 'delete', args)
    }

    async doRequest(url, method, args) {
        try {
            console.log(
                'url=' + url + ', method=' + method +
                ', args: ' + JSON.stringify(args));
            var res;
            if (method === 'post') {
                // POST 方式
                if (args === undefined) {
                    res = await this.axios.post(url);
                } else {
                    console.log(`post-args: ${qs.stringify(args)}`)
                    res = await this.axios.post(url, qs.stringify(args));
                }
            } else if (method === 'delete') {
                // DELETE 方式
                if (args === undefined) {
                    res = await this.axios.delete(url);
                } else {
                    res = await this.axios.delete(url, qs.stringify(args));
                }
            } else {
                // GET 方式
                if (args === undefined) {
                    res = await this.axios.get(url);
                } else {
                    res = await this.axios.get(url, {
                        params: args
                    });
                }
            }

            res = res.data;
            return new Promise(resolve => {
                resolve(res);
            });
        } catch (error) {
            console.error(`server error: ${error}`);
            if (error.response.status === 410) {
                // need to login again
                store.commit(`${USER}/${LOGOUT}`)
            }

            if (error.response) {
                // The request was made and the server responded with a status code
                // that falls out of the range of 2xx
                console.log(error.response.data);
                throw new Error(error.response.data);
            } else if (error.request) {
                // The request was made but no response was received
                // `error.request` is an instance of XMLHttpRequest in the browser and
                // an instance of http.ClientRequest in node.js
                console.log(error.request);
                throw new Error('no response received!');
            } else {
                // Something happened in setting up the request that triggered an Error
                console.log('Error', error.message);
                throw new Error('internal error: ' + error.message);
            }
        }
    }

    static async isSuccessResponse(rsp) {
        return rsp.hasOwnProperty('data') ||
            rsp.hasOwnProperty('status') && rsp.status === 'Ok';
    }
}