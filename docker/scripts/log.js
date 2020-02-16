var log4js = require('log4js')

log4js.configure({
  appenders: {
    out: {
      type: 'stdout',
      layout: {
        type: 'pattern',
        pattern: '%d %p %c %f:%l %m%n'
      }
    },
  },
  categories: {
    default: {
      appenders: ['out'],
      level: 'debug',
      enableCallStack: true,
    }
  },
  disableClustering: true,
})

const logger = log4js.getLogger()

module.exports = logger