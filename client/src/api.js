import Vue from 'vue'
import qs from 'qs'

export default class Api {
  constructor() {
    this.axios = Vue.axios
    // this.host = document.location.host;
    this.host = "http://localhost:8081";
    console.log("host: " + this.host)
    this.config = {
        'Accept': 'application/json',
        'Content-Type': 'application/x-www-form-urlencoded',
        'Access-Control-Allow-Origin': '*'
      },
      this.url = {
        results: `${this.host}/api/results`, // 获取测试结果数据 JSON
        file: `${this.host}/api/file`, // 获取指定文件内容
        list: `${this.host}/api/list`, // 获取USDK测试目录结构
        save: `${this.host}/api/save`, // 保存文件
        description: `${this.host}/api/description`, // 获取用例编号
      }
  }

  async getResults() {
    try {
      let res = await this.axios.get(this.url.results)
      res = res.data
      return new Promise((resolve, reject) => {
        if (res.hasOwnProperty('status')) {
          reject(res)
        } else {
          resolve(res)
        }
      })
    } catch (err) {
      console.error(`server error: ${err}`)
    }
  }

  async getList() {
    try {
      console.log("list url: " + this.url.list);
      let res = await this.axios.get(this.url.list)
      res = res.data
      return new Promise((resolve, reject) => {
        if (res.hasOwnProperty('status')) {
          reject(res)
        } else {
          resolve(res)
        }
      })
    } catch (err) {
      console.error(`server error: ${err}`)
    }
  }

  async getFile(path) {
    try {
      let res = await this.axios.get(
        this.url.file, {
          params: {
            path: path,
          },
          paramsSerializer: function (params) {
            return qs.stringify(params, {
              indices: false
            })
          }
        });
      res = res.data
      return new Promise((resolve, reject) => {
        if (res.hasOwnProperty('status')) {
          reject(res)
        } else {
          resolve(res)
        }
      })
    } catch (err) {
      console.error(`server error: ${err}`)
    }
  }

  async saveFile(path, content) {
    try {
      console.log("saveFile");
      let res = await this.axios.post(
        this.url.save, qs.stringify({
          path: path,
          content: content
        })
      );
      res = res.data
      return new Promise((resolve) => {
        resolve(res)
      })
    } catch (error) {
      console.error(`server error: ${error}`)

      if (error.response) {
        // The request was made and the server responded with a status code
        // that falls out of the range of 2xx
        console.log(error.response.data);
        throw new Error(error.response.data);
      } else if (error.request) {
        // The request was made but no response was received
        // `error.request` is an instance of XMLHttpRequest in the browser and an instance of
        // http.ClientRequest in node.js
        console.log(error.request);
        throw new Error("no response received!");
      } else {
        // Something happened in setting up the request that triggered an Error
        console.log('Error', error.message);
        throw new Error("internal error: " + error.message);
      }

      // return new Promise((reject) => {
      //   if (error.response) {
      //     // The request was made and the server responded with a status code
      //     // that falls out of the range of 2xx
      //     console.log(error.response.data);
      //     reject(error.response.data);
      //   } else if (error.request) {
      //     // The request was made but no response was received
      //     // `error.request` is an instance of XMLHttpRequest in the browser and an instance of
      //     // http.ClientRequest in node.js
      //     console.log(error.request);
      //     reject("no response received!");
      //   } else {
      //     // Something happened in setting up the request that triggered an Error
      //     console.log('Error', error.message);
      //     reject("internal error: " + error.message);
      //   }
      //   console.log(error.config);
      // });
    }
  }

  async getCaseDescription(caseSet, caseId) {
    try {
      console.log("getCaseDescription: (" + caseSet + ", " + caseId + ")");
      let res = await this.axios.get(
        this.url.description, {
          params: {
            set: caseSet,
            id: caseId
          }
        });
      res = res.data
      return new Promise((resolve) => {
        resolve(res)
      })
    } catch (error) {
      console.error(`server error: ${error}`)

      if (error.response) {
        // The request was made and the server responded with a status code
        // that falls out of the range of 2xx
        console.log(error.response.data);
        throw new Error(error.response.data);
      } else if (error.request) {
        // The request was made but no response was received
        // `error.request` is an instance of XMLHttpRequest in the browser and an instance of
        // http.ClientRequest in node.js
        console.log(error.request);
        throw new Error("no response received!");
      } else {
        // Something happened in setting up the request that triggered an Error
        console.log('Error', error.message);
        throw new Error("internal error: " + error.message);
      }
    }
  }


}