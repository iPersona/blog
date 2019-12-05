# API

This document introduce all the APIs used in the blog.

## Visitors

### List all Articles

| TITLE            | List all articles                                                                                                                                                           |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| URL              | article/view_all?limit=:limit&offset:offset                                                                                                                                 |
| PARAMS           | NONE                                                                                                                                                                        |
| METHOD           | GET                                                                                                                                                                         |
| URL_PARAMS       | **Required**:<br/>limit=[integer]<br/>example:limit=1<br/>offset=[integer]<br/>example:offset=10                                                                            |
| DATA_PARAMS      | NONE                                                                                                                                                                        |
| SUCCESS_RESPONSE | `{status: Ok, data: {id: "2494d524-bfcf-4891-a738-468a57ef71d3", title: "Hello world", published: true, create_time: "06/23/2019 10:10", modify_time: "06/23/2019 11:10"}}` |
| ERROR_RESPONSE   | `{status: Err, detail: "invalid user id"}`                                                                                                                                  |
| SAMPLE_CALL      |                                                                                                                                                                             |
| NOTE             |                                                                                                                                                                             |
