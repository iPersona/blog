export default class Utils {
  constructor() {}

  static isValidJsonString(jsonStr) {
    try {
      JSON.parse(jsonStr);
    } catch (e) {
      return false;
    }
    return true;
  }

  static prettyStringify(obj) {
    return JSON.stringify(obj, null, 2)
  }

  static getObjectHash(obj) {
    let hash = require('object-hash');
    let collapseId = '' + hash(obj);
    return collapseId;
  }

  static getLog(caseSet, caseId) {
    return 'logs/log_' + caseSet + '_' + caseId + '.txt';
  }

  static getCaseIdFromQueryString(queryStr) {
    if (queryStr === undefined) {
      return;
    }
    let caseIds = queryStr.split(",");
    return caseIds.map(item => {
      return item.trim();
    });
  }
}