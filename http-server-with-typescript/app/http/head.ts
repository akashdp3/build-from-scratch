import { getMethod, Method, MethodType } from "./method";

export interface IHead {
  method: MethodType;
  path: string;
  protocol: string;
  statusCode?: number;
  headers: Record<string, string>;
}

class Head implements IHead {
  method;
  path;
  protocol;
  headers: Record<string, string>;
  statusCode;

  constructor(requestString: string = "") {
    this.method = Method.GET;
    this.path = "";
    this.protocol = "";
    this.headers = {};
    this.statusCode = 0;

    if (requestString !== "") {
      this.parseHeaderString(requestString);
    }
  }

  setProtocol(protocol: string) {
    this.protocol = protocol;
  }

  setStatusCode(statusCode: number) {
    this.statusCode = statusCode;
  }

  addHeader(headerKey: string, headerValue: string) {
    this.headers[headerKey] = headerValue;
  }

  parseHeaderString(headString: string) {
    const [requestInfo, ...headers] = headString.split("\r\n");
    const requestInfoParts = requestInfo.split(" ");

    this.method = getMethod(requestInfoParts[0]);
    this.path = requestInfoParts[1];
    this.protocol = requestInfoParts[2];

    headers.forEach((header: string) => {
      const [key, value] = header.split(":");
      this.headers = {
        ...this.headers,
        [key.trim()]: value.trim(),
      };
    });
  }

  getStatusMessage() {
    switch (this.statusCode) {
      case 200:
        return "OK";
      case 404:
        return "Not Found";
      case 500:
        return "Internal Server Error";
      case 201:
        return "Created";
    }
  }

  toString() {
    const responseInfoString = `${this.protocol} ${this.statusCode} ${this.getStatusMessage()}`;
    const headerString = Object.keys(this.headers).reduce(
      (acc: string, curr: string) => {
        return `${acc}${curr}: ${this.headers[curr]}\r\n`;
      },
      "",
    );

    return `${responseInfoString}\r\n${headerString}`;
  }
}

export default Head;
