import Vue from 'vue'

export default class Log {
  constructor(component) {
    this.component = component;
  }

  async debug(str) {
    // this.component.$log.debug(str);
    Vue.$log.debug(str);
  }

  async error(str) {
    // this.component.$log.error(str);
    Vue.$log.error(str);
  }
}