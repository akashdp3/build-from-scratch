import Request, { IRequest } from "./request";
import Response, { IResponse } from "./response";
import Route, { IRoute } from "./route";

import routes, { handleRouteNotFound } from "../routes";
import { MethodType } from "./method";
import { matchRoute } from "../utils";

export interface IConnection {
  request: IRequest;
  response: IResponse;
  routes: IRoute[];
}

class Connection implements IConnection {
  request: IRequest;
  response: IResponse;
  routes: IRoute[];

  constructor(request: any) {
    this.request = new Request(request.toString("utf-8"));
    this.response = new Response();
    this.routes = [];
    this.addRoutes();
    const headers = this.request.head.headers;

    this.response.setProtocol(this.request?.head.protocol);
    if (headers["Accept-Encoding"]?.includes("gzip")) {
      this.response.header("Content-Encoding", "gzip");
    }
  }

  addRoutes() {
    routes.forEach((route) => {
      this.routes.push(new Route(route.method, route.pattern, route.callback));
    });
  }

  getCallbackForRequest(method: MethodType, path: string) {
    let params = null;
    const route = this.routes.find((route) => {
      params = matchRoute(route.pattern, path);

      return params && route.method === method;
    });
    return { callback: route?.callback, params };
  }

  generateResponse() {
    const method = this.request?.head.method;
    const path = this.request?.head.path;
    const { callback, params } = this.getCallbackForRequest(method, path);

    if (callback) {
      return callback(this.request, this.response, params);
    } else {
      return handleRouteNotFound(this.request, this.response);
    }
  }
}

export default Connection;
