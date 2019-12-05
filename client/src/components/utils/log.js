import Vue from 'vue'

// get name of component calling log function
function getComponentName(instance) {
  let list = instance.$vnode.tag.split('-')
  return list[list.length - 1]
}

function logStr(name, str) {
  return `name: "${name}"  |  ${str}`
}

let Log = {}

Log.install = function (Vue) {
  Vue.prototype.$getLog = function () {
    let name = getComponentName(this)
    return {
      debug: function (str) {
        Vue.$log.debug(logStr(name, str));
      },
      error(str) {
        Vue.$log.error(logStr(name, str));
      },
      info(str) {
        Vue.$log.info(logStr(name, str));
      },
      warn(str) {
        Vue.$log.warn(logStr(name, str));
      },
      fatal(str) {
        Vue.$log.fatal(logStr(name, str));
      }
    }
  }
}

export default Log