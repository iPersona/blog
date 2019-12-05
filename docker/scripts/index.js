const {
  Client
} = require('pg')
const format = require('pg-format')
const inquirer = require('inquirer')

var connect = async function () {
  // postgres://YourUserName:YourPassword@localhost:5432/YourDatabase
  // DATABASE_URL=postgres://postgres:postgres@localhost/blog
  const client = new Client({
    user: 'postgres',
    host: 'localhost',
    database: 'blog',
    password: 'postgres',
    port: 5432,
  })
  // const client = new Client({
  //   connectionString: "postgres://postgres:postgres@localhost/blog"
  // })
  await client.connect()
  console.log('connect successfully!')
  return client
}

var disconnect = function (client) {
  client.end()
}

var execute = async function (client, sql) {
  try {
    console.log(`sql: ${sql}`)
    let res = await client.query(sql)
    console.log(`res: ${JSON.stringify(res)}`)
  } catch (error) {
    console.error(`error: ${error}`)
  }
}

var insertTags = async function (client) {
  const data = [
    ["Vue"],
    ["Computer"],
    ["Rust"],
    ["Algorithm"],
    ["Life"],
    ["Actix-web"],
    ["Diesel"],
    ["Javascript"],
    ["Swift"],
    ["Apple"],
    ["R2D2"],
    ["Postgres"],
    ["Redis"]
  ]
  let sql = format('INSERT INTO tags (tag) VALUES %L', data)
  await execute(client, sql)
}

var queryArticles = async function (client) {
  let sql = 'SELECT * from articles'
  await execute(client, sql)
}

var insertArticles = async function (client) {
  var data = []
  for (var i = 0; i < 10; i++) {
    let title = `title-${i}`
    let raw_content = `# markdown blog-${i}\n -a\n -b`
    let content = `<h1>markdown blog-${i}</h1> <ul> <li>a<br> -b</li> </ul>`
    data.push([title, raw_content, content])
  }

  let sql = format('INSERT INTO articles(title, raw_content, content) VALUES %L', data)
  await execute(client, sql)
}

let publishArticles = async function (client) {
  let sql = 'UPDATE articles SET published=true'
  await execute(client, sql)
}

const ACTIONS_INSERT_ARTICLES = 'insert articles'
const ACTIONS_INSERT_TAGS = 'insert tags'
const ACTIONS_PUBLISH_ARTICLES = 'publish articles'

let menu = async function () {
  let action = await inquirer
    .prompt([{
      type: 'list',
      name: 'action',
      message: 'What action to take?',
      choices: [
        ACTIONS_INSERT_ARTICLES,
        ACTIONS_INSERT_TAGS,
        ACTIONS_PUBLISH_ARTICLES
      ]
    }])
  return action.action
}

let doAction = async function (client, action) {
  let actions = {
    [ACTIONS_INSERT_ARTICLES]: insertArticles,
    [ACTIONS_INSERT_TAGS]: insertTags,
    [ACTIONS_PUBLISH_ARTICLES]: publishArticles,
  }
  await actions[action](client)
}

var main = async function () {
  let client = await connect()
  let action = await menu()
  console.log(`action: ${action}`)
  // let action = ACTIONS_INSERT_TAGS
  await doAction(client, action)
  disconnect(client)
}

main()