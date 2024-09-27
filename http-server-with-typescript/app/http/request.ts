import Head, { IHead } from "./head";

export interface IRequest {
  head: IHead;
  body: string;
}

class Request implements IRequest {
  head;
  body;

  constructor(requestStringUTF8: string) {
    const requestParts = requestStringUTF8.split("\r\n\r\n");

    this.head = new Head(requestParts[0]);
    this.body = requestParts[1];
  }
}

export default Request;
