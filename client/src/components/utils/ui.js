let Ui = {}

Ui.install = function (Vue) {
  Vue.prototype.$getUi = function () {
    let self = this
    return {
      toast: {
        success: function (msg, isBottom) {
          if (isBottom === undefined) {
            self.$options.parent.$buefy.toast.open({
              message: msg,
              type: 'is-success'
            })
          } else {
            self.$options.parent.$buefy.toast.open({
              message: msg,
              position: 'is-bottom',
              type: 'is-success'
            })
          }
        },
        fail: function (msg, isBottom) {
          if (isBottom === undefined) {
            Vue.$buefy.toast.open({
              message: msg,
              type: 'is-danger'
            })
          } else {
            Vue.$buefy.toast.open({
              message: msg,
              position: 'is-bottom',
              type: 'is-danger'
            })
          }
        }

      }
    }
  }
}
export default Ui