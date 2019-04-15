import qs from 'qs';
import Vue from 'vue';

export default class Api {
  constructor() {
    this.axios = Vue.axios;
    // this.host = document.location.host;
    this.host = 'http://localhost:8880';
    console.log('host: ' + this.host);
    (this.config = {
      Accept: 'application/json',
      'Content-Type': 'application/x-www-form-urlencoded',
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Credentials': true
    }),
        (this.url = {
          results: `${this.host}/api/results`,  // 获取测试结果数据 JSON
          file: `${this.host}/api/file`,        // 获取指定文件内容
          list: `${this.host}/api/list`,        // 获取USDK测试目录结构
          save: `${this.host}/api/save`,        // 保存文件
          description: `${this.host}/api/description`,  // 获取用例描述
          task: `${
              this.host}/api/task`,  // 获取任务执行状态（用例的执行状态列表）
          taskinfo: `${
              this.host}/api/taskinfo`,  // 获取任务执行状态（用例的执行状态列表）
          control: `${
              this.host}/api/control`,  // 获取任务执行状态（用例的执行状态列表）
          reloadcases: `${this.host}/api/reloadcases`,  // 将用例载入到数据库中
          login: `${this.host}/api/v1/user/login`,           // 登录
          createArticle: `${this.host}/api/v1/article/new`,  // 创建文章
          deleteArticle: `${this.host}/article/delete`,      // 删除文章
          adminView: `${this.host}/article/admin/view`,      // 获取文章html
          adminViewRaw: `${this.host}/article/admin/view_raw`,  // 获取文章md
          adminViewAll: `${this.host}/article/admin/view_all`,  // 获取文章列表
          editArticle: `${this.host}/article/edit`,             // 编辑文章
          publishArticle: `${this.host}/api/v1/article/publish`,  // 发布文章
          visitorViewAll:
              `${this.host}/api/v1/article/view_all`,  // 游客：访问文章列表
        });
    // request拦截器
        this.axios.interceptors.request.use(req => {
      // Do something before request is sent
      localStorage.token = '666-66-6666-66666';
      if (localStorage.token !== undefined) {
        req.headers['X-Token'] =
            localStorage  // 让每个请求携带token--['X-Token']为自定义key
                          // 请根据实际情况自行修改
      }
      return req
        }, error => {
            // Do something with request error
            console.log(error) // for debug
            Promise.reject(error)
        });
  }

  async login(account, password, remember) {
    return this.post(
        this.url.login,
        {account: account, password: password, remember: remember});
  }

  async createArticle(title, rawContent, existTags, newTags) {
    let args = {title: title, raw_content: rawContent};
    if (existTags !== undefined) {
      args.exist_tags = existTags;
    }
    if (newTags !== undefined) {
      args.new_tags = newTags;
    }
    return this.post(this.url.createArticle, args);
  }

  async deleteArticle(id) {
    return this.post(this.url.deleteArticle, {id: id});
  }

  async adminView(id) {
    return this.get(this.url.adminView, {id: id});
  }

  async adminViewRawArticle(id) {
    return this.get(this.url.adminViewRaw, {id: id});
  }

  async adminViewAll(limit, offset) {
    return this.get(this.url.adminViewAll, {limit: limit, offset: offset});
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



  async getResults() {
    return this.get(this.url.results);
  }

  async getList() {
    return this.get(this.url.list);
  }

  async getFile(path) {
    return this.get(this.url.file, {path: path});
  }

  async saveFile(path, content) {
    return this.post(this.url.save, {path: path, content: content});
  }

  async getCaseDescription(caseSet, caseId) {
    return this.get(this.url.description, {set: caseSet, id: caseId});
  }

  async getTask() {
    return this.get(this.url.task);
  }

  async getTaskInfo(caseSet, caseId) {
    return this.get(this.url.taskinfo, {set: caseSet, id: caseId});
  }

  async controlTask(start, taskType, cases) {
    if (cases === undefined) {
      return this.post(this.url.control, {start: start, taskType: taskType});
    }

    console.log('controlTask: cases: ' + JSON.stringify(cases));
    return this.post(
        this.url.control,
        {start: start, taskType: taskType, cases: JSON.stringify(cases)});
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
          res = await this.axios.post(url, qs.stringify(args));
        }
      } else {
        // GET 方式
        if (args === undefined) {
          res = await this.axios.get(url);
        } else {
          res = await this.axios.get(url, {params: args});
        }
      }

      res = res.data;
      return new Promise(resolve => {
        resolve(res);
      });
    } catch (error) {
      console.error(`server error: ${error}`);

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
}