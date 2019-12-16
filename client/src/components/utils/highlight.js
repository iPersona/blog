// 参考连接：
// https://hqweay.cn/2019/04/02/marked/

import Hljs from 'highlight.js'

import 'highlight.js/styles/github.css'
import './theme.css'

let HighLight = {};

HighLight.install = function (Vue) {
  Vue.directive('highlight', function (el) {
    let blocks = el.querySelectorAll('pre code');
    blocks.forEach((block) => {
      Hljs.highlightBlock(block)
    })
  });
}

export default HighLight;