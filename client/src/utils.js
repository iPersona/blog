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
}