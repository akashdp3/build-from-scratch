import * as zlib from "zlib";

import Head, { IHead } from "./head";

export interface IResponse {
  head: IHead;
  body: string;
  statusCode: number;
  setProtocol: (protocol: string) => void;
  status: (statusCode: number) => Response;
  header: (headerKey: string, headerValue: string) => Response;
  toString: () => string[];
}

class Response implements IResponse {
  head;
  body;
  statusCode;

  constructor() {
    this.head = new Head();
    this.body = "";
    this.statusCode = 0;
  }

  setProtocol(protocol: string) {
    this.head.setProtocol(protocol);
  }

  status(statusCode: number) {
    this.head.setStatusCode(statusCode);

    return this;
  }

  header(headerKey: string, headerValue: string) {
    this.head.addHeader(headerKey, headerValue);

    return this;
  }

  content(bodyText: string) {
    if (this.head.headers["Content-Encoding"] === "gzip") {
      this.body = zlib.gzipSync(bodyText);
    } else {
      this.body = bodyText;
    }

    return this;
  }

  toString() {
    return [this.head.toString(), this.body];
  }
}

export default Response;
