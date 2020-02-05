export default class Url {
  static getHostUrl() {
    return document.location.host;
  }

  static getUrls() {
    return {
      user: function (userId) {
        return `/#/user/${userId}`
      }
    }
  }
}