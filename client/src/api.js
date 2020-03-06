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
    static api_base_url() {
        // url = document.location.host;
        let url = 'http://localhost:8880/api/v1'
        console.log('host: ' + url);
        return url
    }

    static api_urls() {
        let host = Api.api_base_url()
        return {
            // article
            createArticle: `${host}/article`, // [admin] create article
            deleteArticle: `${host}/article`, // [admin] delete article
            // adminView: `${host}/article/admin/view`, // [admin] get article
            // adminViewRaw: `${host}/article/admin/view_raw`, // [admin] get article markdown
            // adminViewAll: `${host}/article/admin/view_all`, // [admin] get article list
            editArticle: `${host}/article`, // edit article
            publishArticle: `${host}/article`, // publish article
            visitorViewAll: `${host}/articles`, // get article list
            visitorViewArticle: `${host}/article`, // get article
            // articleNumber: `${host}/article/count`, // article count
            // articleNumberByTag: `${host}/article/tag/count`,
            articlesWithTag: function (tagId) {
                return `${host}/tag/${tagId}/articles`
            },

            // comment
            comments: function (articleId) {
                return `${host}/comments/${articleId}`
            },
            newComment: `${host}/comment`,
            locateComment: `${host}/location/comment`,

            // user
            user: function (user_id) {
                return `${host}/user/${user_id}`
            }, // user page
            login: `${host}/login`, // login
            signup: `${host}/user`, // register
            userExist: `${host}/user`, // check whether user exists
            editProfile: `${host}/user`, // edit user profile
            updatePassword: `${host}/user/password`, // update password
            verify: `${host}/verify`, // verify user

            // tag
            getTags: `${host}/tag`,
            getTagsWithCount: `${host}/tags/articles/count`,
            addTags: `${host}/tag`,
            delTag: `${host}/tag`,
            editTag: `${host}/tag`,
            updateTags: `${host}/tags`,

            // statistic
            getDailyVisit: `${host}/dashboard/visit`,

            // notifications
            getCommentNotifications: `${host}/notification/comment`
        }
    }

    constructor(vue) {
        this.axios = Vue.axios;
        this.vue = vue;
        this.config = {
            Accept: 'application/json',
            'Content-Type': 'application/x-www-form-urlencoded',
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Credentials': true
        }
        this.url = Api.api_urls()
        // request拦截器
        this.axios.interceptors.request.use(req => {
            // Do something before request is sent
            // console.log(`localStorage: ${JSON.stringify(localStorage[STORE_KEY])}`)
            if (localStorage[STORE_KEY] !== undefined) {
                // add token into headers
                req.headers.Authorization = store.getters[`user/${TOKEN}`];
                // console.log(`token: ${JSON.stringify(req.headers.Authorization)}`)
            }
            return req
        }, error => {
            // Do something with request error
            console.log(error) // for debug
            Promise.reject(error)
        })
    }

    async getCommentNotifications() {
        return this.get(this.url.getCommentNotifications)
    }

    async updatePassword(oldPassword, newPassword) {
        return this.patch(this.url.updatePassword, {
            old_password: oldPassword,
            new_password: newPassword,
        })
    }

    async editProfile(profile) {
        return this.put(this.url.editProfile, {
            nickname: profile.nickname,
            say: profile.sign,
            email: profile.email,
        })
    }

    async getDailyPeriod(start, end) {
        return this.get(this.url.getDailyVisit, {
            start,
            end
        })
    }

    async locateComment(articleId, commentId, pageSize) {
        return this.get(this.url.locateComment, {
            article_id: articleId,
            comment_id: commentId,
            page_size: pageSize,
        })
    }

    async newComment(articleId, content, userId) {
        return this.post(this.url.newComment, {
            comment: content,
            article_id: articleId,
            reply_user_id: userId
        })
    }

    async getComments(articleId, args) {
        let url = this.url.comments(articleId)
        return this.get(url, args)
    }

    // async getArticleNumberByTag(tagId) {
    //     return this.get(this.url.articleNumberByTag, {
    //         tag_id: tagId
    //     })
    // }

    async getArticlesByTag(tagId, limit, offset) {
        return this.get(this.url.articlesWithTag(tagId), {
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
        return this.put(this.url.updateTags, args)
    }

    async editTag(tagObj) {
        return this.patch(this.url.editTag, tagObj)
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

    async login(account, password, remember, token) {
        return this.post(
            this.url.login, {
                account: account,
                password: password,
                remember: remember,
                token: token,
            });
    }

    async verify(token) {
        return this.post(
            this.url.verify, {
                token: token,
            })
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
        let url = `${this.url.deleteArticle}/${id}`;
        return this.delete(url);
    }

    // async adminView(id) {
    //     return this.get(this.url.adminView, {
    //         id: id
    //     });
    // }

    // async adminViewRawArticle(id) {
    //     return this.get(this.url.adminViewRaw, {
    //         id: id
    //     });
    // }

    // async adminViewAll(limit, offset) {
    //     return this.get(this.url.adminViewAll, {
    //         limit: limit,
    //         offset: offset
    //     });
    // }

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
        return this.put(this.url.editArticle, args);
    }

    async publishArticle(id) {
        return this.patch(this.url.publishArticle, {
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

    // async getArticleNumber() {
    //     return this.get(this.url.articleNumber);
    // }

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
        return this.post(this.url.reloadcases)
    }

    async post(url, args) {
        return this.doRequest(url, 'post', args)
    }

    async get(url, args) {
        return this.doRequest(url, 'get', args);
    }

    async delete(url, args) {
        return this.doRequest(url, 'delete', args)
    }

    async patch(url, args) {
        return this.doRequest(url, 'patch', args)
    }

    async put(url, args) {
        return this.doRequest(url, 'put', args)
    }

    async doRequest(url, method, args) {
        try {
            console.log(
                'url=' + url + ', method=' + method +
                ', args: ' + JSON.stringify(args));
            var res;
            if (args === undefined) {
                res = await this.axios[method](url);
            } else {
                console.log(`post-args: ${qs.stringify(args)}`)
                let params = method === 'get' ? {
                        params: args
                    } :
                    qs.stringify(args)
                res = await this.axios[method](url, params);
            }
            res = res.data;

            // error handler
            Api.errorHandler(res);

            return new Promise(resolve => {
                resolve(res);
            });
        } catch (error) {
            console.error(`server error: ${error}`);
            // if (error.response.status === 410) {
            //     // need to login again
            //     store.commit(`${USER}/${LOGOUT}`)
            // }

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

    static isSuccessResponse(rsp) {
        return rsp.hasOwnProperty('data') ||
            rsp.hasOwnProperty('status') && rsp.status === 'Ok';
    }

    static isKindOfError(rsp, code) {
        return rsp.hasOwnProperty('status') &&
            rsp.status === 'Err' &&
            rsp.code === code
    }

    static errorHandler(rsp) {
        if (Api.isKindOfError(rsp, 'TokenExpired')) {
            // need to login again
            store.commit(`${USER}/${LOGOUT}`)
        } else if (Api.isKindOfError(rsp, 'EmailNotVerified')) {
            // redirect to error page
            let url = `/error?title=Email 未验证&detail=请先验证 Email 激活账户！`
            let encodedUri = encodeURIComponent(url)
            console.log(`encoded uri: ${encodedUri}`)
            window.location.replace(encodedUri)
        }
    }
}