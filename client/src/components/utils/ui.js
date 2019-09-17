export default class Ui {
  constructor(component) {
    this.component = component;
  }

  async toastSuccess(msg) {
    this.component.$buefy.toast.open({
      message: msg,
      type: 'is-success'
    });
  }

  async toastFail(msg) {
    this.component.$buefy.toast.open({
      message: msg,
      type: 'is-danger'
    });
  }
}