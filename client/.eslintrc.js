module.exports = {
  // https://stackoverflow.com/questions/36002226/parsing-error-the-keyword-import-is-reserved-sublimelinter-contrib-eslint
  parserOptions: {
    parser: "babel-eslint"
  },
  extends: ["plugin:vue/recommended", "plugin:vue/base"],
  rules: {
    'no-console': 'off',
  }
};