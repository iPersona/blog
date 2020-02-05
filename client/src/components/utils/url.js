export default class Url {
  static getHostUrl() {
    return document.location.host;
  }

  static getUrls() {
    return {
      user: function (userId) {
        return `${Url.getHostUrl()}/#/user/${userId}`
      }
    }
  }
}