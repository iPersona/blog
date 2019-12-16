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
}