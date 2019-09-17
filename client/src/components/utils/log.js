export default class Log {
  constructor(component) {
    this.component = component;
  }

  async debug(str) {
    this.component.$log.debug(str);
  }

  async error(str) {
    this.component.$log.debug(str);
  }
}