export default class DatetimeUtil {
  constructor() {}

  static toDateTime(datetime) {
    let dateFormat = require('dateformat')
    let d = Date.parse(datetime)
    return dateFormat(d, "yyyy-mm-dd")
  }

  static toTimeAgo(datetime) {
    let locale = window.navigator.userLanguage || window.navigator.language;
    let moment = require('moment')
    moment.locale(locale)
    return moment.utc(datetime).local().fromNow()
  }
}