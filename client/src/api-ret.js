import {
  PERMISSION_DENIED
} from '@/error-code'

export default class ApiRet {
  constructor(rsp) {
    // [data]: represent the data of the api
    // [status]: represent the status of the api
    // [code]: error code
    // detail: error detail
    this.rsp = rsp
  }

  isSuccess() {
    return this.rsp.hasOwnProperty('data') ||
      this.rsp.hasOwnProperty('status') && this.rsp.status === 'Ok'
  }

  data() {
    return this.rsp.data
  }

  errorCode() {
    return this.rsp.code
  }

  errorDetail() {
    return this.rsp.detail
  }

  isKindOfError(code) {
    return this.rsp.hasOwnProperty('status') &&
      this.rsp.status === 'Err' &&
      this.rsp.code === code
  }

  isPermissionDenied() {
    return this.isKindOfError(PERMISSION_DENIED)
  }
}