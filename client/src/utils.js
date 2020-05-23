export default class Utils {
  constructor() {}

  static randomString(len) {
    len = len || 32;
    let chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var maxPos = chars.length;
    var pwd = '';
    for (var i = 0; i < len; i++) {
      //0~32的整数
      pwd += chars.charAt(Math.floor(Math.random() * (maxPos + 1)));
    }
    return pwd;
  }

  static password(password) {
    return Utils.randomString(6) + password
  }

  static isObjEmpty(obj) {
    return obj === undefined ||
      (Object.entries(obj).length === 0 && obj.constructor === Object)
  }

  static isStringEmpty(str) {
    return !str
  }

  static blink(callback, delay, repetitions) {
    Utils.startStoppableInterval(callback, delay, repetitions)
  }

  static startStoppableInterval(callback, delay, repetitions) {
    let x = 0
    let intervalID = setInterval(function () {
      callback()
      if (++x === repetitions) {
        clearInterval(intervalID)
      }
    }, delay)
  }
}